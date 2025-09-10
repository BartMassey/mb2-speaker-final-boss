At 50% duty cycle (?)

0. ?
1. Measurement setup.
2. 440Hz 1ms/div 1V/div
3. 880Hz 
4. 1760Hz
5. 1760Hz 200us/div 1V/div
6. 3520Hz
7. 7040Hz
8. 7040Hz 50us/div 500mV/div
9. 12544Hz
10. 220Hz 2ms/div 1V/div (bad amplitude stability)
11. 110Hz
12. 55Hz
13. 55Hz 5ms/div 1V/div
14. 26Hz 10ms/div 1V/div
15. 16Hz 20ms/div 200mV/div (peaks clipped)
16. 62Hz 5ms/div 800mV/div (min f for 50% duty cycle)

volume range

440 Hz (key 69): 21-34%
1976 Hz (key 95): 21-34%
220 Hz (key 57): 18-27% (buzzing)
110 Hz (key 45): 16-24% (loud buzzing)

duty cycle at 440Hz (key 69)

17. 20% 1ms/div 100mV/div
18. 22%
19. 23% 200mV/div
20. 25% 500mV/div
21. 27% 1V/div
22. 28% (no further perceived loudness after this)
23. 30% 

duty cycle at 1976Hz (key 95)

24. 21% 200us/div 100mV/div (lowest perceivable audio, width 58/253)
25. 23%
26. 25% 500mV/div
27. 28% (no further perceived loudness after this, width 70/253)
28. 30%
29. 50%

duty cycle at 12544Hz (key 127)

30. 24% 50us/div 500mV/div (width 9/39)
31. 29% (width 11/39)
32. 31% (width 12/39)

Note that the duty cycle here is clearly way higher than
indicated, for some unknown reason. Hypothesis 1: hitting
switching speed limits in the microcontroller or
circuit. Hypothesis 2: bug in PWM driver.

