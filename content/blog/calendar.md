= Calendar
:date: 2020-08-03
:draft:

* Reactivity is my bushel of wheat: A problem that I've attempted many times and now give up for lost whenever I see it.
* Layout
	* All: Require custom keyboard navigation
	* Table:
		* Automatic LTR / RTL support
		* Multi day events are hard: col-span
	* CSS Grid:
		* Automatic LTR / RTL support (grid layout changes direction automatically)
		* 
	* SVG:
		* Need manual LTR / RTL support
		* Can have true squares without hacks
		* Easier mental model (coordinates)
		* Manual focus control
		* For single day view: easier to add a current time marker
* Dates
	* Date arithmetic is hard: Daylight savings time (not all days are the same length)
	* Luxon
	* Built-in formatting
	* Switching between long / short date styles based on available space - ResizeObserver
	* 
