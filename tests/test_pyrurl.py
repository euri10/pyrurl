from pyrurl.pyrurl import parse_url

from pyrurl import __version__


def test_version():
    assert __version__ == '0.1.0'


def test_sum():
    from urllib.parse import urlparse
    google = 'http://google.fr'
    a = parse_url(google)
    b = urlparse(google)
    print(a)
    print(b)
