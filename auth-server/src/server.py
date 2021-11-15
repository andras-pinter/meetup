import json

from dataclasses import dataclass
from http.server import SimpleHTTPRequestHandler
from password import PasswordStore


@dataclass
class Response:
    code: int
    reason: str

    def dump(self) -> str:
        return json.dumps({
            "code": self.code,
            "reason": self.reason,
        })


class AuthServer(SimpleHTTPRequestHandler):
    def do_GET(self):
        store = PasswordStore()
        try:
            body = json.loads(self.rfile.read(int(self.headers["Content-Length"])).decode("utf-8"))
            self.send_response(200)
            self.send_header("Content-type", "application/json")
            self.end_headers()
            if body.get("password") == store.password():
                self.log_message(f"Password rotated to: {store.rotate()}")
                resp = Response(0x0, "Authorized")
            else:
                self.log_message("Incorrect password")
                resp = Response(0x1, "Incorrect password")
            self.wfile.write(resp.dump().encode())
            self.wfile.flush()
        except json.decoder.JSONDecodeError:
            self.log_message("Invalid JSON request format")
            self.send_response(400)
            self.end_headers()
