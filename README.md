# wave
Implementation of cycloidal wave equations for discrete spaces

The following works for as many spatio-temporal dimension as you want.

Those kind of wave are usually described by a lot of points moving arround circles according to this system:

x(t) = φ(t)
y(t) = R*sin(t/R)

with φ(t) = t + R*cos(t/R)
R the radius of the circles described by the points (the magnitude of the wave)

I needed to apply this in a case where the hyperplane the wave propagate onto is discretized
(there are only a handfull of constant x instead of an infinity) and so I needed y with some *really precise* x.

So, the goal was to get "z(x) = the height at the point x"

Meaning I needed to find the inverse function of φ.
Lucky, φ is a bijection (in this case), meaning there actually /is/ an inverse function.
But after a bit of researches, turned out that it couldn't be expressed using standart functions
(like exp, cos, sin, +, - ...).

So let Φ = φ^-1

We would have
Φ(x) = t
z(x) = y(Φ(x))=  R*sin(Φ(t)/R)

And that would totaly work just fine.

I opted for a simple dichotomic algorithm to approach Φ(x), with an arbitrary precision.
For now, it works in O( log(1/precision) ) but I may find something even better with more mathematical tinkering.

And then I got borred and tried to display all of that in a fancy way.


# How to run

get cargo and a terminal
>cargo run<
and there it should process a bit a create a lot of images in images/
Then you can use ffmpeg or something to make it into a git
(because animations are greate to visualize things that are supposed to *move*)
