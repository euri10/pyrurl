import pytest
from pyrurl.pyrurl import parse_url

from pyrurl import __version__


def test_version():
    assert __version__ == '0.1.0'


@pytest.mark.parametrize("url", [
    # ("http://google.fr"),
    # ('http://www.cwi.nl/%7Eguido/Python.html'),
    # ('http://user:pass@NetLoc:80/path;parameters/path2;parameters2?query=argument#fragment'),
    # # no user
    # ('http://:pass@NetLoc:80/path;parameters/path2;parameters2?query=argument#fragment'),
    # no pass
    ('http://user:@NetLoc:80/path;parameters/path2;parameters2?query=argument#fragment'),
])
def test_sum(url):
    from urllib.parse import urlparse
    rustlib = parse_url(url)
    pylib = urlparse(url)
    print("\n")
    print(rustlib.__repr__())
    print(pylib)
    assert rustlib.scheme == pylib.scheme
    assert rustlib.netloc == pylib.netloc
    assert rustlib.path == pylib.path
    assert rustlib.params == pylib.params
    assert rustlib.query == pylib.query
    assert rustlib.fragment == pylib.fragment
