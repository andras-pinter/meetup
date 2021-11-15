from http.server import HTTPServer
from server import AuthServer

PORT = 8080

if __name__ == "__main__":
    server = HTTPServer(("", PORT), AuthServer)
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        server.server_close()
