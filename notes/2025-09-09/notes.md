Decided to try sine waves today. Will probably also be a
buzzy nonlinear mess, but that would be progress.

Got the old audio experiments code [mb2-audio-experiments]
working again (`speaker-output-2025` branch). Recorded some
audio [mb2-linus-audio] from it using the Behringer
reference microphone and Scarlett 2i2. Very faint, barely
recognizable.

Fired up the square wave generator again. Added button
gating, ran it and looked at a reference recording
[mb2-usquare-audio]. Results were not promising. Time domain
[usquare-signal] did not look the least bit squarish, and
frequency domain [usquare-spectrum] was noisy in the extreme
(although my recording environment was very loud). It looked
like the signal was ringing heavily.

[mb2-audio-experiments]: https://github.com/pdx-cs-rust-embedded/mb2-audio-experiments
[mb2-linus-audio]: audio/mb2-linus-audio.wav
[mb2-usquare-audio]: audio/mb2-usquare-audio.wav
[usquare-signal]: images/usquare-signal.png
[usquare-spectrum]: images/usquare-spectrum.png
