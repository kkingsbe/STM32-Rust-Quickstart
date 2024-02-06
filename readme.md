## J-Link Setup Instructions:
1. Download Zedig from [here](https://zadig.akeo.ie/)
2. Install the WinUSB driver
3. (Still within Zedig) replace the J-Link devices driver with WinUSB
4. Verify your setup by running `probe-rs-cli info`. This should not report an error.

## Debugging:
1. Flash code onto chip as usual
2. Run `probe-rs-cli debug --chip stm32f411ceux` to initialize a debugging session