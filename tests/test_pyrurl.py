import pytest
from pyrurl.pyrurl import parse_url

from pyrurl import __version__


def test_version():
    assert __version__ == '0.1.0'


@pytest.mark.parametrize("url", [
    ("http://google.fr"),
    ('http://www.cwi.nl/%7Eguido/Python.html'),
])
def test_sum(url):
    from urllib.parse import urlparse
    a = parse_url(url)
    b = urlparse(url)
    print(a.__repr__())
    print(b)
