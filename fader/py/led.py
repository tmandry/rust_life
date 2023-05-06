import board
import digitalio
import time

led = digitalio.DigitalInOut(board.LED)
led.direction = digitalio.Direction.OUTPUT

#print("Hello, World!")

import neopixel
from rainbowio import colorwheel
rgb = neopixel.NeoPixel(board.NEOPIXEL, 1)
rgb.brightness = 0.1

i = 0
while True:
    i = (i + 2) % 256
    rgb.fill(colorwheel(i))
    led.value = True
    time.sleep(0.1)
    led.value = False
    time.sleep(0.1)
