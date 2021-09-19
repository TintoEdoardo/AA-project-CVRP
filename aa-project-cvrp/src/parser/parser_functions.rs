/*
 * Functions used to parse the content of
 * the specification file.
 */

pub(crate) fn parse_line(&line : String) -> void
{



}

fn skip_whitespaces(&line : String) -> String
{

    let len : u8 = line.len();
    let first_char : u8;

    for i in len
    {

        if char::is_whitespace(line[i])
        {

            continue;

        }

        first_char = i;
        break;

    }

    let string_without_initial_ws : String = line[first_char..len];

}

pub(crate) fn get_token(&line : String) -> (String, String)
{

    /* Remove eventual whitespaces. */
    let line_without_initial_ws : String = skip_whitespaces(line);

    /* Compute index for token. */
    let len : u8       = line_without_initial_ws.len() as u8;
    let mut last_char : u8 = 0;

    for i in len
    {
        let c : char = line[i];

        if char::is_whitespace(c)
        {

            last_char = i;
            break;

        }

    }

    let token : String          = line_without_initial_ws[0..last_char];
    let remaining_line : String = line_without_initial_ws[last_char..len];

    (token, remaining_line)

}