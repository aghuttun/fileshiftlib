use ssh2::Session;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SftpError {
    #[error("SSH error: {0}")]
    SshError(String),
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    #[error("Connection error: {0}")]
    ConnectionError(String),
    #[error("Authentication error: {0}")]
    AuthError(String),
}

impl From<ssh2::Error> for SftpError {
    fn from(err: ssh2::Error) -> Self {
        SftpError::SshError(err.to_string())
    }
}

pub struct SftpConfiguration {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

impl SftpConfiguration {
    pub fn new(host: String, port: u16, username: String, password: String) -> Self {
        Self {
            host,
            port,
            username,
            password,
        }
    }
}

pub struct SftpClient {
    config: SftpConfiguration,
    session: Option<Session>,
}

impl SftpClient {
    /// Create a new SFTP client instance.
    pub fn new(host: String, username: String, password: String, port: u16) -> Self {
        let config = SftpConfiguration::new(host, port, username, password);
        Self {
            config,
            session: None,
        }
    }

    /// Authenticate and establish SFTP session.
    pub fn authenticate(&mut self) -> Result<(), SftpError> {
        let tcp = TcpStream::connect(format!("{}:{}", self.config.host, self.config.port))
            .map_err(|e| SftpError::ConnectionError(e.to_string()))?;

        let mut session = Session::new()
            .map_err(|e| SftpError::SshError(e.to_string()))?;

        session.set_tcp_stream(tcp);
        session.handshake()
            .map_err(|e| SftpError::SshError(e.to_string()))?;

        session.userauth_password(&self.config.username, &self.config.password)
            .map_err(|e| SftpError::AuthError(e.to_string()))?;

        self.session = Some(session);
        Ok(())
    }

    /// Reconnect to the SFTP server.
    pub fn reconnect(&mut self) -> Result<(), SftpError> {
        self.session = None;
        self.authenticate()
    }

    /// Check if the connection is active.
    pub fn is_connected(&self) -> bool {
        self.session.is_some()
    }

    /// List directory contents.
    pub fn list_dir(&self, path: &str) -> Result<Vec<String>, SftpError> {
        let session = self.session.as_ref()
            .ok_or_else(|| SftpError::ConnectionError("Not connected".to_string()))?;

        let sftp = session.sftp()
            .map_err(|e| SftpError::SshError(e.to_string()))?;

        let entries = sftp.readdir(Path::new(path))
            .map_err(|e| SftpError::SshError(e.to_string()))?;

        let names = entries
            .into_iter()
            .filter_map(|(path, _stat)| {
                path.file_name()
                    .and_then(|n| n.to_str())
                    .map(|s| s.to_string())
            })
            .collect();

        Ok(names)
    }

    /// Change directory.
    pub fn change_dir(&self, path: &str) -> Result<(), SftpError> {
        let session = self.session.as_ref()
            .ok_or_else(|| SftpError::ConnectionError("Not connected".to_string()))?;

        let sftp = session.sftp()
            .map_err(|e| SftpError::SshError(e.to_string()))?;

        sftp.opendir(Path::new(path))
            .map_err(|e| SftpError::SshError(e.to_string()))?;

        Ok(())
    }

    /// Delete a file.
    pub fn delete_file(&self, filename: &str) -> Result<(), SftpError> {
        let session = self.session.as_ref()
            .ok_or_else(|| SftpError::ConnectionError("Not connected".to_string()))?;

        let sftp = session.sftp()
            .map_err(|e| SftpError::SshError(e.to_string()))?;

        sftp.unlink(Path::new(filename))
            .map_err(|e| SftpError::SshError(e.to_string()))?;

        Ok(())
    }

    /// Download a file from the remote server.
    pub fn download_file(&self, remote_path: &str, local_path: &str) -> Result<(), SftpError> {
        let session = self.session.as_ref()
            .ok_or_else(|| SftpError::ConnectionError("Not connected".to_string()))?;

        let sftp = session.sftp()
            .map_err(|e| SftpError::SshError(e.to_string()))?;

        let mut remote_file = sftp.open(Path::new(remote_path))
            .map_err(|e| SftpError::SshError(e.to_string()))?;

        let mut local_file = std::fs::File::create(local_path)?;

        let mut contents = Vec::new();
        remote_file.read_to_end(&mut contents)
            .map_err(|e| SftpError::SshError(e.to_string()))?;

        local_file.write_all(&contents)?;

        Ok(())
    }

    /// Upload a file to the remote server.
    pub fn upload_file(&self, local_path: &str, remote_path: &str) -> Result<(), SftpError> {
        let session = self.session.as_ref()
            .ok_or_else(|| SftpError::ConnectionError("Not connected".to_string()))?;

        let sftp = session.sftp()
            .map_err(|e| SftpError::SshError(e.to_string()))?;

        let mut local_file = std::fs::File::open(local_path)?;
        let mut remote_file = sftp.create(Path::new(remote_path))
            .map_err(|e| SftpError::SshError(e.to_string()))?;

        let mut contents = Vec::new();
        local_file.read_to_end(&mut contents)?;
        remote_file.write_all(&contents)
            .map_err(|e| SftpError::SshError(e.to_string()))?;

        Ok(())
    }
}

impl Drop for SftpClient {
    fn drop(&mut self) {
        // Session will be closed automatically when dropped
        self.session = None;
    }
}
