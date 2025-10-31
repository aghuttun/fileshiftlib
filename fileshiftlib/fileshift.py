"""
Provide an SFTP client class for connecting to, authenticating with, and interacting with an SFTP server.

This module defines the `SFTP` class, which enables file operations such as listing directories, changing directories,
uploading, downloading, and deleting files on a remote SFTP server.
It uses the `paramiko` library for secure file transfer and supports logging for all operations.

Classes
-------
SFTP
    Encapsulate SFTP connection management and file operations.
"""

from dataclasses import dataclass
import logging
import paramiko

# Creates a logger for this module
logger = logging.getLogger(__name__)


class SFTP(object):
    """
    Manage SFTP connections and perform file operations on a remote SFTP server.

    Use this class to connect to an SFTP server, authenticate, and perform file operations such as listing directories,
    changing directories, uploading, downloading, and deleting files.
    Leverage Paramiko for secure file transfer and logging for all operations.

    Parameters
    ----------
    host : str
        Hostname or IP address of the SFTP server.
    username : str
        Username for authentication.
    password : str
        Password for authentication.
    port : int, optional
        Port number of the SFTP server (default is 22).
    logger : logging.Logger, optional
        Logger instance to use. If None, create a default logger.

    Attributes
    ----------
    _logger : logging.Logger
        Logger for SFTP operations.
    _configuration : Configuration
        SFTP connection configuration.
    _transport : paramiko.Transport
        Underlying Paramiko transport object.
    sftp_client : paramiko.SFTPClient
        Paramiko SFTP client for file operations.

    Methods
    -------
    auth()
        Authenticate with the SFTP server and initialize the SFTP client.
    reconnect()
        Reconnect to the SFTP server by closing and re-authenticating the session.
    is_connected()
        Check if the SFTP connection is currently active.
    list_dir(path=".")
        List the names of the contents in the specified folder on the SFTP server.
    change_dir(path=".")
        Change the current working directory on the SFTP server.
    delete_file(filename)
        Delete a file on the SFTP server.
    download_file(remote_path, local_path)
        Download a file from the SFTP server to the local machine.
    upload_file(local_path, remote_path)
        Upload a file from the local machine to the SFTP server.
    """

    @dataclass
    class Configuration:
        """
        Store SFTP connection configuration parameters.

        Parameters
        ----------
        host : str, default="10.0.0.1"
            Specify the hostname or IP address of the SFTP server.
        port : int, default=22
            Set the port number of the SFTP server.
        username : str, default="admin"
            Provide the username for authentication.
        password : str or None, default=None
            Provide the password for authentication or set to None if not required.
        """

        host: str = "10.0.0.1"
        port: int = 22
        username: str = "admin"
        password: str | None = None

    def __init__(
        self,
        host: str,
        username: str,
        password: str,
        port: int = 22,
        logger: logging.Logger | None = None,
    ) -> None:
        """
        Initialize the SFTP client with the given configuration and authenticate.

        Parameters
        ----------
        host : str
            Specify the hostname or IP address of the SFTP server.
        username : str
            Provide the username for authentication.
        password : str
            Provide the password for authentication.
        port : int, optional
            Set the port number of the SFTP server. Default is 22.
        logger : logging.Logger, optional
            Supply a logger instance to use. If None, create a default logger.
        """
        # Init logging
        # Use provided logger or create a default one
        self._logger = logger or logging.getLogger(name=__name__)

        # Credentials/configuration
        self._configuration = self.Configuration(host=host,
                                                 port=port,
                                                 username=username,
                                                 password=password)

        # Authenticate
        self._transport, self.sftp_client = self.auth()

    def __del__(self) -> None:
        """
        Clean up the SFTP client and close the transport session.

        Close the SFTP transport and SFTP client to release resources and terminate the session.

        Notes
        -----
        This method is called automatically when the object is deleted.
        """
        self._logger.info(msg="Closing the SFTP session and releasing resources.")

        self._transport.close()
        self.sftp_client.close()

    def auth(self) -> tuple:
        """
        Authenticate with the SFTP server and initialize the SFTP client.

        Log the start of the session, establish a connection to the SFTP server using the provided configuration, and
        create an SFTP client for file operations.

        Returns
        -------
        tuple
            A tuple containing the Paramiko transport and SFTP client objects.
        """
        self._logger.info(msg="Establishing a new SFTP session with the remote server.")

        # Connect
        transport = paramiko.Transport((self._configuration.host, self._configuration.port))
        transport.connect(username=self._configuration.username, password=self._configuration.password)
        sftp_client = paramiko.SFTPClient.from_transport(transport)

        return transport, sftp_client

    def reconnect(self) -> None:
        """
        Reconnect to the SFTP server by closing the current session and re-authenticating.

        Log the reconnection attempt, close the existing transport and SFTP client if they are open, and establish a
        new authenticated session.

        Raises
        ------
        Exception
            If an error occurs while closing the existing connection.
        """
        self._logger.info(msg="Establishing a secure connection and authenticating with the SFTP server.")

        try:
            self._transport.close()
            self.sftp_client.close()
        except Exception as e:
            self._logger.warning(msg=f"Error closing existing connection: {e}")

        self._transport, self.sftp_client = self.auth()

    def is_connected(self) -> bool:
        """
        Check if the SFTP connection is currently active.

        Returns
        -------
        bool
            True if the connection is active, otherwise False.
        """
        self._logger.info(msg="Checking if the SFTP connection is currently active.")

        return self._transport.is_active()

    def list_dir(self, path: str = ".") -> list:
        """
        List the names of the contents in the specified folder on the SFTP server.

        Parameters
        ----------
        path : str, optional
            Path to the folder on the SFTP server. Defaults to the current directory.

        Returns
        -------
        list of str
            Names of the contents in the specified folder.
        """
        self._logger.info(msg="Listing the names of the contents in the specified folder on the SFTP server.")
        self._logger.info(msg=path)

        return self.sftp_client.listdir(path)

    def change_dir(self, path: str = ".") -> None:
        """
        Change the current working directory on the SFTP server.

        Parameters
        ----------
        path : str, optional
            Path to the folder to change to on the SFTP server. Defaults to the current directory.

        Returns
        -------
        None

        See Also
        --------
        list_dir : List the contents of a directory on the SFTP server.
        """
        self._logger.info(msg="Changing the current working directory on the SFTP server.")
        self._logger.info(msg=path)

        self.sftp_client.chdir(path)

    def delete_file(self, filename: str) -> None:
        """
        Delete a file on the SFTP server.

        Parameters
        ----------
        filename : str
            Name of the file to delete on the SFTP server.

        Returns
        -------
        None

        See Also
        --------
        download_file : Download a file from the SFTP server.
        upload_file : Upload a file to the SFTP server.
        """
        self._logger.info(msg="Deleting a file from the SFTP server.")
        self._logger.info(msg=filename)

        self.sftp_client.remove(filename)

    def download_file(self, remote_path: str, local_path: str) -> None:
        """
        Download a file from the SFTP server to the local machine.

        Parameters
        ----------
        remote_path : str
            Path to the file on the SFTP server.
        local_path : str
            Path on the local machine where the file will be saved.

        Returns
        -------
        None

        See Also
        --------
        upload_file : Upload a file from the local machine to the SFTP server.
        """
        self._logger.info(msg="Downloading a file from the SFTP server to the local machine.")
        self._logger.info(msg=remote_path)
        self._logger.info(msg=local_path)

        self.sftp_client.get(remote_path, local_path)

    def upload_file(self, local_path: str, remote_path: str) -> None:
        """
        Upload a file from the local machine to the SFTP server.

        Parameters
        ----------
        local_path : str
            Specify the path to the file on the local machine.
        remote_path : str
            Specify the path on the SFTP server where the file will be saved.

        Returns
        -------
        None

        See Also
        --------
        download_file : Download a file from the SFTP server to the local machine.
        delete_file : Delete a file on the SFTP server.
        """
        self._logger.info(msg="Uploading a file from the local machine to the SFTP server.")
        self._logger.info(msg=local_path)
        self._logger.info(msg=remote_path)

        self.sftp_client.put(local_path, remote_path)


# eof
