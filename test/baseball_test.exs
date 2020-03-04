defmodule BaseballTest do
  use ExUnit.Case
  doctest Baseball

  test "greets the world" do
    assert Baseball.hello() == :world
  end
end
