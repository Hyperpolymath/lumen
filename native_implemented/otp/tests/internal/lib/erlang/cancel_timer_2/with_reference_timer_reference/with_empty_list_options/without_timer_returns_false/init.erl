-module(init).
-export([start/0]).
-import(erlang, [cancel_timer/2, display/1]).

start() ->
  TimerReference = test:reference(),
  Options = [],
  display(cancel_timer(TimerReference, Options)).
