




import http.server
import socketserver

PORT = 8000
DIRECTORY = "/path/to/files"

Handler = http.server.SimpleHTTPRequestHandler
Handler.directory = DIRECTORY

with socketserver.TCPServer(("", PORT), Handler) as httpd:
    print(f"Server running on port {PORT}")
    httpd.serve_forever()









