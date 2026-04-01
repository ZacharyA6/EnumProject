from enum import Enum

class GameState(Enum):
    START = 1
    PLAYING = 2
    PAUSED = 3
    GAME_OVER = 4
    RESTART = 5


def handle_state(state):
        if state == GameState.START:
            print("Game is starting...")

        elif state == GameState.PLAYING:
            print("Game is in progress!")

        elif state == GameState.PAUSED:
            print("Game is paused.")

        elif state == GameState.GAME_OVER:
            print("Game over!")

        else:
            print("Unknown state")

current_state = GameState.START
handle_state(current_state)

current_state = GameState.PLAYING
handle_state(current_state)

current_state = GameState.PAUSED
handle_state(current_state)

current_state = GameState.GAME_OVER
handle_state(current_state)