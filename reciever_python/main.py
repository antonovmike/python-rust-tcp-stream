import socket
import threading


def listen():
    serversocket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    serversocket.bind(("localhost", 8080))
    serversocket.listen(2)

    while True:
        (clientsocket1, address1) = serversocket.accept()
        (clientsocket2, address2) = serversocket.accept()

        ct1 = client_thread(clientsocket1)
        ct1.start()
        ct2 = client_thread(clientsocket2)
        ct2.start()


class client_thread(threading.Thread):
    def __init__(self, clientsocket):
        threading.Thread.__init__(self)
        self.clientsocket = clientsocket

    def run(self):
        while True:
            data = self.clientsocket.recv(1024)
            if not data:
                break
            print(data)


if __name__ == "__main__":
    listen()
