import nalog

def test_log(capfd):
    logger = nalog.Logger("testing")
    logger.warning("First!")
    captured = capfd.readouterr()
    assert captured.out == "[testing] First!\n"
    assert not captured.err
