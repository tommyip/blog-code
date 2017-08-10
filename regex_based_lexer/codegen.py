"""
Generate a large source file for benchmarking.
"""

import random
import string


def gen_ident(lo, hi):
    return ''.join([
        random.choice(string.ascii_lowercase)
        for _ in range(random.randint(lo, hi))
    ])


def gen_string(lo, hi):
    #                                                   > space hack
    characters = string.ascii_letters + string.digits + '            '
    return ''.join([
        random.choice(characters)
        for _ in range(random.randint(lo, hi))
    ])


def gen_number():
    return random.randint(1, 1000000)


def gen_line():
    return random.choice([
        '{} = {} + "{}" * ({} / {})'.format(
            gen_ident(1, 5), gen_ident(1, 5), gen_string(1, 50), gen_number(), gen_number()),
        '{} = {}'.format(gen_ident(1, 10), gen_number()),
        '// {}'.format(gen_string(20, 70)),
        '{} = "{}" * {}'.format(gen_ident(1, 10), gen_string(5, 40), gen_number())
    ])


if __name__ == '__main__':
    src = '\n'.join([gen_line() for _ in range(1000000)])
    with open("src_file", 'w') as f:
        f.write(src)
