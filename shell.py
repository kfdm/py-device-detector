import py_device_detector
import code
import datetime

print("Creating dd")
dd = py_device_detector.DeviceDetector(128)


def test(ua):
    print("Testing :: ", ua)
    start = datetime.datetime.now()
    result = dd.parse(ua)
    end = datetime.datetime.now()
    print("Result :: ", result)
    print("Time   :: ", end - start)


test("Testing")
test("Safari")
test("Chrome")
code.interact(local={"py_device_detector": py_device_detector, "dd": dd})
