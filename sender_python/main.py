import socket
import time

TCP_IP = '127.0.0.1'
TCP_PORT = 6006
BUFFER_SIZE = 96
MESSAGE = "Hello, World! (Python)"

s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.connect((TCP_IP, TCP_PORT))

while True:
    s.send(MESSAGE.encode())
    time.sleep(10)
