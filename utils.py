def ascii_progress_bar(
    progress : float, # between 0 - 1
    char_length : int = 24,
    empty_char : str = ".",
    filled_char : str = "█"
) -> str:
    filled_squares = int(char_length * progress)
    empty_squares = char_length - filled_squares
    return '[' + (filled_char * filled_squares) + (empty_char * empty_squares) + ']'