import time
import board
import pwmio
import analogio
import touchio
import neopixel

fader = analogio.AnalogIn(board.A0)
while True:
    reading = fader.value

    fader_pos = reading // 256
    fader_width = 90  # for text visualization in serial output
    print("-" * (fader_width - int(fader_pos/3)), fader_pos, "-" * int(fader_pos/3), reading, " "*10, end='\r')

    time.sleep(0.5)
