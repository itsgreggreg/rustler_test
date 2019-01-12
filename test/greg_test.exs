defmodule GregTest do
  use ExUnit.Case
  doctest Greg

  test "greets the world" do
    assert Greg.hello() == :world
  end
end
