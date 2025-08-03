from flask import Flask, jsonify
from flask_cors import CORS
import sqlite3
import os

app = Flask(__name__)
CORS(app)

DB_PATH = os.path.join(os.path.dirname(__file__), 'database.db')

@app.route("/players")
def get_players():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    players = conn.execute("SELECT * FROM players").fetchall()
    conn.close()
    return jsonify([dict(row) for row in players])

if __name__ == "__main__":
    app.run(port=5000)