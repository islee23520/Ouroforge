defmodule OuroforgeExecutor.JSON do
  @moduledoc false

  def decode!(input) when is_binary(input) do
    input
    |> parse_value()
    |> case do
      {value, rest} ->
        case skip_ws(rest) do
          "" ->
            value

          extra ->
            raise ArgumentError,
                  "unexpected JSON trailing data: #{inspect(String.slice(extra, 0, 24))}"
        end
    end
  end

  defp parse_value(input) do
    input = skip_ws(input)

    case input do
      <<?{, rest::binary>> -> parse_object(skip_ws(rest), %{})
      <<?[, rest::binary>> -> parse_array(skip_ws(rest), [])
      <<?", _::binary>> -> parse_string(input)
      <<"true", rest::binary>> -> {true, rest}
      <<"false", rest::binary>> -> {false, rest}
      <<"null", rest::binary>> -> {nil, rest}
      <<ch, _::binary>> when ch in ~c"-0123456789" -> parse_number(input)
      _ -> raise ArgumentError, "invalid JSON value near #{inspect(String.slice(input, 0, 24))}"
    end
  end

  defp parse_object(<<?}, rest::binary>>, acc), do: {acc, rest}

  defp parse_object(input, acc) do
    {key, rest} = parse_string(skip_ws(input))
    rest = skip_ws(rest)

    case rest do
      <<?:, rest::binary>> ->
        {value, rest} = parse_value(rest)
        rest = skip_ws(rest)
        acc = Map.put(acc, key, value)

        case rest do
          <<?,, rest::binary>> ->
            parse_object(skip_ws(rest), acc)

          <<?}, rest::binary>> ->
            {acc, rest}

          _ ->
            raise ArgumentError,
                  "expected object comma or close near #{inspect(String.slice(rest, 0, 24))}"
        end

      _ ->
        raise ArgumentError, "expected object colon near #{inspect(String.slice(rest, 0, 24))}"
    end
  end

  defp parse_array(<<?], rest::binary>>, acc), do: {Enum.reverse(acc), rest}

  defp parse_array(input, acc) do
    {value, rest} = parse_value(input)
    rest = skip_ws(rest)

    case rest do
      <<?,, rest::binary>> ->
        parse_array(skip_ws(rest), [value | acc])

      <<?], rest::binary>> ->
        {Enum.reverse([value | acc]), rest}

      _ ->
        raise ArgumentError,
              "expected array comma or close near #{inspect(String.slice(rest, 0, 24))}"
    end
  end

  defp parse_string(<<?", rest::binary>>) do
    parse_string_chars(rest, [])
  end

  defp parse_string(input) do
    raise ArgumentError, "expected JSON string near #{inspect(String.slice(input, 0, 24))}"
  end

  defp parse_string_chars(<<?", rest::binary>>, acc),
    do: {acc |> Enum.reverse() |> IO.iodata_to_binary(), rest}

  defp parse_string_chars(<<?\\, ?", rest::binary>>, acc),
    do: parse_string_chars(rest, [?" | acc])

  defp parse_string_chars(<<?\\, ?\\, rest::binary>>, acc),
    do: parse_string_chars(rest, [?\\ | acc])

  defp parse_string_chars(<<?\\, ?/, rest::binary>>, acc),
    do: parse_string_chars(rest, [?/ | acc])

  defp parse_string_chars(<<?\\, ?b, rest::binary>>, acc),
    do: parse_string_chars(rest, [?\b | acc])

  defp parse_string_chars(<<?\\, ?f, rest::binary>>, acc),
    do: parse_string_chars(rest, [?\f | acc])

  defp parse_string_chars(<<?\\, ?n, rest::binary>>, acc),
    do: parse_string_chars(rest, [?\n | acc])

  defp parse_string_chars(<<?\\, ?r, rest::binary>>, acc),
    do: parse_string_chars(rest, [?\r | acc])

  defp parse_string_chars(<<?\\, ?t, rest::binary>>, acc),
    do: parse_string_chars(rest, [?\t | acc])

  defp parse_string_chars(<<?\\, ?u, hex::binary-size(4), rest::binary>>, acc) do
    {codepoint, ""} = Integer.parse(hex, 16)
    parse_string_chars(rest, [<<codepoint::utf8>> | acc])
  rescue
    _ -> raise ArgumentError, "invalid JSON unicode escape"
  end

  defp parse_string_chars(<<ch::utf8, rest::binary>>, acc),
    do: parse_string_chars(rest, [<<ch::utf8>> | acc])

  defp parse_string_chars("", _acc), do: raise(ArgumentError, "unterminated JSON string")

  defp parse_number(input) do
    {token, rest} = take_number(input, "")

    value =
      if String.contains?(token, [".", "e", "E"]) do
        String.to_float(token)
      else
        String.to_integer(token)
      end

    {value, rest}
  end

  defp take_number(<<ch, rest::binary>>, acc) when ch in ~c"-+0123456789.eE" do
    take_number(rest, <<acc::binary, ch>>)
  end

  defp take_number(rest, acc) when acc != "", do: {acc, rest}

  defp skip_ws(<<ch, rest::binary>>) when ch in ~c" \n\r\t", do: skip_ws(rest)
  defp skip_ws(rest), do: rest
end
