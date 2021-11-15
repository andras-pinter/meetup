from random import choice


class PasswordStore:
    _instance = None
    _store = ["alma", "korte", "malac", "szilva"]

    def __new__(cls, *args, **kwargs):
        if cls._instance is None:
            cls._instance = super().__new__(cls)
            try:
                cls._instance._pass = cls._store[0]
            except KeyError:
                cls._instance._pass = "root"
        return cls._instance

    def password(self) -> str:
        return self._pass

    def rotate(self):
        self._pass = choice(self._store)
        return self._pass
