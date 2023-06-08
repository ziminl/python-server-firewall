


import socket
blocked_ips = ["1.1.1.1", "8.8.8.8"]
server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
server_address = ('', 8000)
server_socket.bind(server_address)
server_socket.listen(1)
print("wait for responce")
while True:
    client_socket, client_address = server_socket.accept()
    print("Received connection from:", client_address)
    if client_address[0] in blocked_ips:
        print("Blocked connection from:", client_address)
        client_socket.close()
        continue
    response = "Hello, client!"
    client_socket.sendall(response.encode())
    client_socket.close()
    
    
    
    
