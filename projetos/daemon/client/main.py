from MainWindow import MainWindow
from PySide6.QtWidgets import QApplication
import sys
import socket

if __name__ == '__main__':
    try:
        client_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        server_address = ("127.0.0.1", 8080)
        client_socket.connect(server_address)

        app = QApplication(sys.argv)
        window = MainWindow(client_socket)
        window.show()
        sys.exit(app.exec())
    except:
        print("Erro ao conectar-se com o servidor.")
        sys.exit(1)