use pyo3::prelude::*;
use pyo3::exceptions::PyException;
use crate::models::{SftpClient, SftpError};

fn sftp_error_to_pyexc(err: SftpError) -> PyErr {
    PyException::new_err(err.to_string())
}

/// SFTP client for connecting to and interacting with SFTP servers.
#[pyclass(name = "SFTP")]
pub struct PySftpClient {
    inner: SftpClient,
}

#[pymethods]
impl PySftpClient {
    /// Initialize the SFTP client with the given configuration.
    ///
    /// Parameters
    /// ----------
    /// host : str
    ///     Hostname or IP address of the SFTP server.
    /// username : str
    ///     Username for authentication.
    /// password : str
    ///     Password for authentication.
    /// port : int, optional
    ///     Port number of the SFTP server (default is 22).
    /// logger : object, optional
    ///     Logger instance (for API compatibility, currently unused).
    #[new]
    #[pyo3(signature = (host, username, password, port = 22, logger = None))]
    fn new(
        host: String,
        username: String,
        password: String,
        port: u16,
        logger: Option<PyObject>,
    ) -> PyResult<Self> {
        let _ = logger; // Ignore logger for now (for API compatibility)
        
        let mut client = SftpClient::new(host, username, password, port);
        client.authenticate().map_err(sftp_error_to_pyexc)?;

        Ok(Self { inner: client })
    }

    /// Reconnect to the SFTP server.
    fn reconnect(&mut self) -> PyResult<()> {
        self.inner.reconnect().map_err(sftp_error_to_pyexc)
    }

    /// Check if the SFTP connection is currently active.
    ///
    /// Returns
    /// -------
    /// bool
    ///     True if the connection is active, otherwise False.
    fn is_connected(&self) -> bool {
        self.inner.is_connected()
    }

    /// List the names of the contents in the specified folder on the SFTP server.
    ///
    /// Parameters
    /// ----------
    /// path : str, optional
    ///     Path to the folder on the SFTP server. Defaults to the current directory.
    ///
    /// Returns
    /// -------
    /// list of str
    ///     Names of the contents in the specified folder.
    #[pyo3(signature = (path = "."))]
    fn list_dir(&self, path: &str) -> PyResult<Vec<String>> {
        self.inner.list_dir(path).map_err(sftp_error_to_pyexc)
    }

    /// Change the current working directory on the SFTP server.
    ///
    /// Parameters
    /// ----------
    /// path : str, optional
    ///     Path to the folder to change to on the SFTP server. Defaults to the current directory.
    #[pyo3(signature = (path = "."))]
    fn change_dir(&self, path: &str) -> PyResult<()> {
        self.inner.change_dir(path).map_err(sftp_error_to_pyexc)
    }

    /// Delete a file on the SFTP server.
    ///
    /// Parameters
    /// ----------
    /// filename : str
    ///     Name of the file to delete on the SFTP server.
    fn delete_file(&self, filename: &str) -> PyResult<()> {
        self.inner.delete_file(filename).map_err(sftp_error_to_pyexc)
    }

    /// Download a file from the SFTP server to the local machine.
    ///
    /// Parameters
    /// ----------
    /// remote_path : str
    ///     Path to the file on the SFTP server.
    /// local_path : str
    ///     Path on the local machine where the file will be saved.
    fn download_file(&self, remote_path: &str, local_path: &str) -> PyResult<()> {
        self.inner.download_file(remote_path, local_path).map_err(sftp_error_to_pyexc)
    }

    /// Upload a file from the local machine to the SFTP server.
    ///
    /// Parameters
    /// ----------
    /// local_path : str
    ///     Path to the file on the local machine.
    /// remote_path : str
    ///     Path on the SFTP server where the file will be saved.
    fn upload_file(&self, local_path: &str, remote_path: &str) -> PyResult<()> {
        self.inner.upload_file(local_path, remote_path).map_err(sftp_error_to_pyexc)
    }
}
