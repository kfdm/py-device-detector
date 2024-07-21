import py_device_detector
import code
import datetime
from pprint import pformat

print("Creating dd")
dd = py_device_detector.DeviceDetector(128)


def test(ua):
    print('*'*80)
    print("Testing :: ", ua)
    start = datetime.datetime.now()
    result = dd.parse(ua)
    end = datetime.datetime.now()
    print("Result :: ", pformat(result))
    print("Time   :: ", end - start)
    print('*'*80)


test("Testing")
test("Googlebot/2.1 (+http://www.google.com/bot.html)")
test("Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:128.0) Gecko/20100101 Firefox/128.0")
test("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.5 Safari/605.1.15")
code.interact(local={"py_device_detector": py_device_detector, "dd": dd})
