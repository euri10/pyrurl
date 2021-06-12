import pytest
import whatwg_url
from yarl import URL as YarlURL

from pyrurl.pyrurl import parse_url

from pyrurl import __version__


def test_version():
    assert __version__ == '0.1.0'


@pytest.mark.parametrize("url", [
    ("http://google.fr"),
    ('http://www.cwi.nl/%7Eguido/Python.html'),
    ('http://user:pass@NetLoc:80/path;parameters/path2;parameters2?query=argument#fragment'),
    # no user
    ('http://:pass@NetLoc:80/path;parameters/path2;parameters2?query=argument#fragment'),
    # no pass
    ('http://user:@NetLoc:80/path;parameters/path2;parameters2?query=argument#fragment'),
])
class TestPythonUrlLibraries:

    def test_vs_yarl(self, url):
        pyrurl_result = parse_url(url)
        yarlurl_result = YarlURL(url)
        print("\n")
        print(pyrurl_result.__repr__())
        print(yarlurl_result)
        print()
        assert pyrurl_result.scheme == yarlurl_result.scheme
        assert pyrurl_result.path == yarlurl_result.path
        # assert rustlib.params == yarllib.params
        assert pyrurl_result.query == yarlurl_result.query_string
        assert pyrurl_result.fragment == yarlurl_result.fragment

    def test_vs_whatwg_url(self, url):
        pyrurl_result = parse_url(url)
        whatwg_url_result = whatwg_url.urlparse(url)
        print("\n")
        print(pyrurl_result.__repr__())
        print(whatwg_url_result)
        print()
        assert pyrurl_result.scheme == whatwg_url_result.scheme
        assert pyrurl_result.path == whatwg_url_result.path
        # assert rustlib.params == yarllib.params
        assert pyrurl_result.query == whatwg_url_result.query
        assert pyrurl_result.fragment == whatwg_url_result.fragment


NOT_A_VALID_URL = "htt"


def test_exception():
    with pytest.raises(TypeError, match="Error parsing! relative URL without a base"):
        parse_url(NOT_A_VALID_URL)