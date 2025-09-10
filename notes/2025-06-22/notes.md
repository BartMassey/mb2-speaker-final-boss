Output the PWM to an external pin to validate what I was
seeing. It was… instructive. Turns out that the duty cycle
was inverted by default, as I had thought but wasn't seeing.
All the duty cyles I've been using were thus 100 - d where d
is the duty cycle I *thought* I was using.

Hypothesis: the quieting is happening by leaving the speaker
connected each cycle for long enough to drain the power
capacitor. This makes successive cycles have less power
available. Evidence: actual duty cycles below 50% all sound
pretty much the same until about 1%.

Referencing my speaker measurements to ground instead of
speaker negative was really interesting. With PWM off both
the speaker high side (SHS) and speaker low side (SLS) are
at 3.3V as expected. With PWM on at 10% width, SHS clearly
shows a discharge curve. SLS shows the grounding pulse from
the MOSFET; once released it follows the SHS curve.

1. 440 Hz 10% 1V/div 1ms/div yellow=SHS blue=SLS

At 3% width, the volume is almost maximal, apparently due to
the big spike at the start of each cycle.

2. 3% yellow=500mV/div blue=2V/div

At 50% the SHS discharge curve is pretty well gone, but the
SLS curve now shows a charging curve.

3. 50% yellow=200mV/div blue=1V/div

The quieting action starts at about 73%, where the SLS
starts to fail to make 3.3V before the next cycle.

4. 73%

By 75% the speaker is quite a bit quieter.

5. 75%, yellow=500mV/div

By 78% the speaker is very quiet. The speaker is
experiencing very little differential drive: both SHS and
SLS are near 3.3VDC.

Going back to differential mode, we note that at 70% the
speaker just can stay energized through the MOSFET on time.

6. 70% 1V/div

At 71% the circuit starts to discharge a bit before the end
of the on cycle.

7. 71%

At 72% the circuit is discharging mid-cycle.

8. 72%

At 73% the circuit discharge starts to happen early. Note
that starting voltage is now notably lower.

9. 73%

At this point, small increases in pulse width will make big
power changes.

10. 75%

By 78% the signal is nearly inaudible.

11. 78%

Ok, all this justifies the volume hack. The speaker circuit
operation is still pretty impenetrable to me; I'm going to
have to get some help.

Meanwhile, let's go up to a high frequency — say 20KHz — and
see what the scope shows us. At 70%, we have pretty much
full apparent amplitude across the cycle.

12. 20KHz 70% 500mV/div 20us/div

By 72% we are starting to lose power.

13. 72%

By 74% power is down substantially.

14. 74%

By 77% we have lost all but the negative-going spike.

15. 77%

By 79% we are essentially silent.

20KHz is the ultrasonic range, at least for me. So now the
obvious next thing to do is to try the uber-experiment. We
will try to make a 440Hz wave by 20KHz PWM of the speaker in
this narrow width range.

---

OK! We have a fairly loud square wave at about 440Hz
produced by 20KHz PWM. I'm happy so far. The weird part is
that I'm switching between a 10% and 90% duty cycle and
it's louder that way. So apparently something is still a bit
off. The waveform shows the expected drop and rise during
the high periods: I think that's unavoidable. The square
wave is really buzzy, but probably not much to be done about
that.

16. mod-square 10%/90%, 200us/div, 1V/div

Moving back to 65%/85% produces a *much* quieter wave
without much to recommend it. I'm declaring fail.

17. mod-square 65%/85%, 200us/div, 500mV/div
