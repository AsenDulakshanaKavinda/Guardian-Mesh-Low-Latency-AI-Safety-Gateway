from .handle_logging import log
from functools import wraps


class ThreatClassificationException(Exception):
    def __init__(self, message: str, component: str, error_code: int):
        super().__init__(message)
        self.component = component
        self.error_code = error_code
        log.error(f"Execution Error in [{component}] (Code: {error_code}): {message}")


def handle_errors(component_name: str, error_code: int):
    def decorator(func):
        @wraps(func)
        def wrapper(*args, **kwargs):
            try:
                return func(*args, **kwargs)
            except Exception as e:
                error_msg = f"Error in {component_name}: {str(e)}"
                log.error(error_msg)
                raise ThreatClassificationException(
                    message=error_msg,
                    component=component_name,
                    error_code=error_code
                ) from e
            return wrapper
        return decorator