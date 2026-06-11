from functools import wraps


def my_decorator(func):
    @wraps(func)
    def wrapper(*args, **kwargs):
        print("Something is happening before the fn is called.")
        return func(*args, **kwargs)
    return wrapper

@my_decorator
def say_hello():
    """This function says hello."""
    print("Hello!")


# Let's check the identity of say_hello
print(say_hello.__name__)       # Output: wrapper
print(say_hello.__doc__)        # Output: None (instead of "This function says hello.")





