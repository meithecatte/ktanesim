import random
import modules

class Switches(modules.Module):
    identifiers = ['switches']
    display_name = "Switches"
    manual_name = "Switches"
    help_text = "`{cmd} flip 1 5 3 4 2 5 2` or `{cmd} flip 1534252`. Partial solutions allowed."
    module_score = 4

    # Switch states are represented as binary numbers. For example,
    #     UP DOWN UP UP DOWN
    # would be encoded as
    #     0b10110 = 22
    # This is used since it's a compact notation for invalid positions that
    # can be also efficiently manipulated.

    # Input: switch number from 0 to 4
    # Output: a bitmask with the corresponding bit set
    def bitmask_for_switch(self, switch):
        return 1 << (4 - switch)

    invalid_positions = {
        0b00100, 0b01011, 0b01111, 0b10010, 0b10011,
        0b10111, 0b11000, 0b11010, 0b11100, 0b11110,
    }

    def state_as_string(self, state):
        return " ".join("UP" if self.bitmask_for_switch(i) & state != 0 else "DOWN"
                        for i in range(5))

    def __init__(self, bomb, ident):
        super().__init__(bomb, ident)

        valid_positions = set(range(0b11111 + 1)) - self.invalid_positions
        self.solution = random.choice(list(valid_positions))

        # Make sure that the initial position requires at least 2 switches
        # to be flipped

        valid_positions.remove(self.solution)
        for i in range(5):
            valid_positions.discard(self.solution ^ self.bitmask_for_switch(i))

        self.position = random.choice(list(valid_positions))

        self.log(f"Initial position: {self.state_as_string(self.position)}")
        self.log(f"Solution: {self.state_as_string(self.solution)}")

    def generate_switch(self, index):
        up = self.position & self.bitmask_for_switch(index) != 0
        target_up = self.solution & self.bitmask_for_switch(index) != 0
        SWITCH_POLYGON = [(50.5, 174), (65.5, 174), (73, 234), (43, 234)]
        positions = [[x[0] + 58 * index, x[1]] for x in SWITCH_POLYGON]
        if up:
            positions[2][1] = 348 - positions[2][1]
            positions[3][1] = 348 - positions[3][1]

        positions = " ".join(f"{pos[0]},{pos[1]}" for pos in positions)

        o = f'<circle stroke-width="2" stroke="#000" cx="{58*(index+1)}" cy="174" r="15" />'
        o += f'<circle stroke-width="2" stroke="#000" fill="{"#0f0" if target_up else "#fff"}" cx="{58*(index+1)}" cy="87" r="10" />'
        o += f'<circle stroke-width="2" stroke="#000" fill="{"#0f0" if not target_up else "#fff"}" cx="{58*(index+1)}" cy="261" r="10" />'
        o += f'<polygon points="{positions}" style="fill:black;stroke:black;stroke-width:2" />'

        return o

    def get_svg(self, led):
        output = (f'<svg viewBox="0 0 348 348" fill="#fff" stroke-linecap="butt" stroke-linejoin="round" stroke-miterlimit="10" xmlns:xlink="http://www.w3.org/1999/xlink">'
                  f'<path stroke="#000" stroke-width="2" d="M5 5h338v338h-338z"/>'
                  f'<circle fill="{led}" stroke="#000" cx="298" cy="40.5" r="15" stroke-width="2"/>')

        for i in range(5):
            output += self.generate_switch(i)
        output += '</svg>'
        return output

    @modules.check_solve_cmd
    async def cmd_flip(self, author, parts):
        if not parts:
            return await self.usage(author)

        parsed = []

        for digit in ''.join(parts):
            if digit.isdigit():
                switch = int(digit)
                if switch not in range(1, 6):
                    return await self.bomb.channel.send(f"{author.mention} There is no switch {switch}! There are five switches: 1-5.")
                parsed.append(switch - 1)
            else:
                return await self.bomb.channel.send(f"{author.mention} No such switch: `{digit}`")

        for switch in parsed:
            bitmask = self.bitmask_for_switch(switch)
            new_position = self.position ^ bitmask
            if new_position in self.invalid_positions:
                self.log(f"Can't flip switch {switch+1} because "
                         f"{self.state_as_string(new_position)} is "
                         f"not a valid position.")
                return await self.handle_strike(author)
            self.position = new_position
            self.log(f"Flipped switch {switch+1}. The position is now "
                     + self.state_as_string(self.position))
            if self.position == self.solution:
                return await self.handle_solve(author)
        await self.do_view(author.mention)

    COMMANDS = {
        "flip": cmd_flip
    }
