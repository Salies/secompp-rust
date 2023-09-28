import socket

# Server address and port
server_address = ("127.0.0.1", 8080)

try:
    # Create a socket object
    client_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

    # Connect to the server
    client_socket.connect(server_address)

    while True:
        # Get user input
        user_input = input("Enter 'a' or 'b' (or 'exit' to quit): ")

        if user_input == "exit":
            break

        # Send the user input to the server
        client_socket.send(user_input.encode())

        # Receive and print the server's response
        response = client_socket.recv(1024).decode()
        print(f"Server response: {response}")

except KeyboardInterrupt:
    print("Client terminated by user.")

finally:
    # Close the socket when done
    client_socket.close()
