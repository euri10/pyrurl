from timeit import Timer
from yarl import URL
from whatwg_url import parse_url as w_parse_url
from pyrurl.pyrurl import parse_url as p_parse_url
from httptools import parse_url

GOOGLE = "https://www.google.com"
GOOGLEB = GOOGLE.encode()
TIMES = 10000


def yarl_def():
    return URL(GOOGLE)


def whatwg_def():
    return w_parse_url(GOOGLE)


def httptools_def():
    return parse_url(GOOGLEB)


def pyrulr_def():
    return p_parse_url(GOOGLE)


def bench():
    yarl_timer = Timer(yarl_def).timeit(TIMES)
    print(f"yarl: {yarl_timer}")

    # whatwg_timer = Timer(whatwg_def).timeit(TIMES)
    # print(whatwg_timer)

    httptools_timer = Timer(httptools_def).timeit(TIMES)
    print(f"httptools: {httptools_timer}")

    pyrurl_timer = Timer(pyrulr_def).timeit(TIMES)
    print(f"pyrurl: {pyrurl_timer}")


if __name__ == '__main__':
    bench()
