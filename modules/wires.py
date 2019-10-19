import random
import itertools
import enum
import modules
import edgework

class Wires(modules.Module):
    identifiers = ['wires']
    display_name = "Wires"
    manual_name = "Wires"
    help_text = "`{cmd} cut 3` to cut the third wire. Empty spaces are not counted."
    module_score = 1
    vanilla = True

    @enum.unique
    class Color(enum.Enum):
        black =  "#000"
        blue =   "#00f"
        red =    "#f00"
        white =  "#fff"
        yellow = "#ff0"

    PATHS_UNCUT = [
        "M65.898438,90.912109 c -1.576082,-0.198603 -3.215872,-0.156976 -4.921876,0.208985 l 1.259766,5.867187 c 4.023083,-0.863006 8.043492,1.43865 12.857422,4.818359 4.81393,3.37971 10.054968,7.72643 16.78125,8.39844 25.11869,2.50953 49.92935,-2.81228 73.95508,-1.40039 4.85979,0.28558 9.41352,3.04832 14.34375,6.45117 4.93022,3.40285 10.13574,7.40432 16.6289,8.98828 25.2538,6.16046 51.03948,9.12443 76.47266,12.75195 l 0.8457,-5.93945 c -25.54045,-3.64283 -51.1276,-6.59845 -75.89648,-12.64062 -4.88126,-1.19075 -9.58377,-4.60606 -14.64258,-8.09766 -5.05881,-3.4916 -10.57237,-7.1007 -17.40039,-7.50195 -25.1198,-1.47619 -49.84318,3.80252 -73.710937,1.41797 -4.31262,-0.43087 -8.931655,-3.82893 -13.929687,-7.337896 -3.748525,-2.631722 -7.914333,-5.388564 -12.642578,-5.984375 z",
        "M79.429688,124.41211 c -5.156519,-0.0461 -10.27896,0.83644 -15.304688,3.12109 l 2.484375,5.46289 c 11.067282,-5.03109 23.279695,-1.92804 36.576175,2.85547 13.29647,4.78351 27.308,11.21787 41.6914,11.05078 24.59358,-0.28569 49.50733,-7.64146 70.92969,-0.16796 8.77888,3.06263 19.72486,10.66295 30.37109,15.88281 5.32312,2.60993 10.6214,4.6592 15.86915,5.07226 5.24774,0.41307 10.56094,-1.05509 14.71289,-5.20703 l -4.24415,-4.24219 c -2.93547,2.93548 -6.02885,3.77923 -9.99804,3.4668 -3.96919,-0.31243 -8.69755,-2.0252 -13.69727,-4.47656 -9.99943,-4.90273 -20.88848,-12.61966 -31.03711,-16.16016 -23.86911,-8.32707 -49.60292,-0.43946 -72.97461,-0.16797 -12.49367,0.14514 -25.98731,-5.80099 -39.59179,-10.69531 -8.502802,-3.05895 -17.192915,-5.7181 -25.787112,-5.79492 z",
        "M93.236328,154.05859 c -3.682182,-0.001 -7.367143,0.58118 -10.96289,1.36524 -7.191495,1.56812 -14.218755,3.95004 -20.587891,4.78515 l 0.78125,5.94922 c 7.225121,-0.94734 14.364494,-3.40742 21.085937,-4.87304 6.721444,-1.46563 12.831094,-1.92708 18.244146,0.41796 1.31363,0.5691 2.40433,1.74556 3.45117,3.62305 1.04683,1.87749 1.94737,4.3451 2.90625,6.89063 0.95887,2.54552 1.9636,5.17601 3.5664,7.48632 1.60281,2.31032 4.0819,4.35726 7.32813,4.79493 30.95881,4.17391 62.07066,-1.60513 92.36133,-4.2168 20.80879,-1.79414 42.25766,8.00586 65.10937,8.00586 v -6 c -21.0513,0 -42.69307,-9.96157 -65.625,-7.98438 -30.68385,2.64557 -61.35175,8.25328 -91.04492,4.25 -1.36836,-0.18448 -2.18586,-0.80885 -3.19922,-2.26953 -1.01336,-1.46068 -1.9467,-3.70174 -2.88086,-6.18164 -0.93415,-2.47989 -1.8829,-5.18933 -3.28125,-7.69726 -1.39835,-2.50793 -3.34766,-4.92514 -6.30664,-6.20703 -3.58386,-1.55261 -7.263129,-2.13736 -10.945312,-2.13868 z",
        "M93.806641,183.77734 c -11.315125,-0.13224 -22.4792,2.98695 -32.875,3.68946 l 0.404297,5.98632 c 14.959168,-1.01088 29.473876,-5.75563 42.312502,-2.4414 10.53733,2.72017 20.92191,9.49114 33.33594,10.89258 22.6526,2.55729 45.40177,2.42578 66.89843,7.46484 11.72512,2.7485 21.48797,5.87484 29.72657,6.19141 7.52568,0.28916 14.95175,-1.70331 21.84765,-2.86328 6.8959,-1.15998 13.04955,-1.49331 18.30859,1.13867 l 2.68555,-5.36524 c -7.08388,-3.54524 -14.72832,-2.91099 -21.99023,-1.68945 -7.26192,1.22154 -14.35456,3.02399 -20.6211,2.7832 -6.82274,-0.26216 -16.62776,-3.23351 -28.58789,-6.03711 -22.42372,-5.25637 -45.42207,-5.08076 -67.5957,-7.58398 -10.54186,-1.19009 -20.67671,-7.68608 -32.50781,-10.74024 -3.78262,-0.97646 -7.570091,-1.3817 -11.341799,-1.42578 z",
        "M101.34375,222.14453 c -11.915491,-0.38005 -23.809415,0.33728 -35.582031,2.51758 l 1.091797,5.90039 c 45.161684,-8.36397 93.271904,5.7762 137.707034,20.58594 3.03639,1.01199 6.09504,0.48029 8.61523,-0.68164 2.52019,-1.16194 4.68543,-2.87707 6.76367,-4.58594 4.15649,-3.41774 7.98075,-6.57535 11.18555,-6.8125 13.57108,-1.00422 27.7339,2.83008 42.57227,2.83008 v -6 c -13.75374,0 -28.07255,-3.91824 -43.01563,-2.8125 -6.16408,0.45613 -10.58272,4.89575 -14.55273,8.16015 -1.98501,1.63221 -3.85206,3.02986 -5.46485,3.77344 -1.61279,0.74358 -2.78716,0.91073 -4.20703,0.4375 -33.42928,-11.14161 -69.36681,-22.17235 -105.11328,-23.3125 z",
        "M135.9082,249.06641 c -6.63407,0 -12.00903,3.43897 -16.78711,7.08007 -4.77807,3.64111 -9.16866,7.58585 -13.63281,9.65821 -6.384118,2.96364 -15.125811,4.48388 -23.103514,3.75 -7.977704,-0.73388 -15.011192,-3.66727 -18.826172,-8.90821 l -4.84961,3.53125 c 5.213707,7.16248 14.032238,10.51493 23.126954,11.35157 9.094715,0.83663 18.646612,-0.7871 26.177732,-4.28321 5.65903,-2.62704 10.26447,-6.91246 14.74414,-10.32617 4.47968,-3.41371 8.62368,-5.85351 13.15039,-5.85351 8.76005,0 17.68419,1.37193 25.71875,4.49609 0.64279,0.24994 1.23113,0.84192 1.88086,2.14844 0.64974,1.30651 1.21441,3.16606 1.81641,5.125 0.602,1.95894 1.22072,4.02182 2.39844,5.91992 1.17771,1.8981 3.32051,3.72029 6.04296,3.93359 33.00535,2.58612 66.99569,0.93388 99.24805,-7.13672 l -1.45508,-5.82031 c -31.47542,7.87618 -64.83765,9.52008 -97.32421,6.97461 -0.6586,-0.0516 -0.82751,-0.17109 -1.41211,-1.11328 -0.58461,-0.94219 -1.17963,-2.61903 -1.76368,-4.51953 -0.58404,-1.9005 -1.17719,-4.01931 -2.17968,-6.03516 -1.00249,-2.01585 -2.56005,-4.08924 -5.07813,-5.06836 -8.87528,-3.45106 -18.5094,-4.90429 -27.89258,-4.90429 z",
    ]

    PATHS_CUT = [
        "M65.898438,90.912109 c -1.576082,-0.198603 -3.215871,-0.156976 -4.921876,0.208985 l 1.259766,5.867187 c 4.023083,-0.863006 8.043492,1.43865 12.857422,4.818359 4.81393,3.37971 10.054967,7.72644 16.78125,8.39844 25.11869,2.50954 49.92935,-2.81228 73.95508,-1.40039 l 0.35156,-5.98828 c -25.11979,-1.47618 -49.84318,3.80253 -73.710937,1.41797 -4.312619,-0.43086 -8.931655,-3.82893 -13.929687,-7.337896 -3.748525,-2.631722 -7.914333,-5.388564 -12.642578,-5.984375 z m 132.326172,27.503911 -1.42188,5.82812 c 25.2538,6.16048 51.03948,9.12442 76.47266,12.75195 l 0.8457,-5.93945 c -25.54045,-3.64283 -51.1276,-6.59844 -75.89648,-12.64062 z",
        "M79.429688,124.41211 c -5.156519,-0.0461 -10.27896,0.83644 -15.304688,3.12109 l 2.484375,5.46289 c 11.067282,-5.03109 23.279695,-1.92804 36.576175,2.85547 13.29647,4.78351 27.30801,11.21787 41.6914,11.05078 l -0.0684,-6 c -12.49367,0.14514 -25.98732,-5.80099 -39.59179,-10.69531 -8.502802,-3.05895 -17.192915,-5.7181 -25.787112,-5.79492 z m 109.509762,12.98047 c -3.24251,0.10601 -6.48805,0.32684 -9.72265,0.60547 -2.02254,0.17421 -4.04115,0.37206 -6.05664,0.58007 l 0.61523,5.96876 c 1.9942,-0.20583 3.98071,-0.40009 5.95703,-0.57032 12.64292,-1.08905 24.82164,-1.1678 36.07422,2.75782 8.77888,3.06263 19.72485,10.66295 30.37109,15.88281 5.32312,2.60993 10.6214,4.6592 15.86915,5.07226 5.24774,0.41307 10.56094,-1.05509 14.71289,-5.20703 l -4.24415,-4.24219 c -2.93547,2.93548 -6.02885,3.77923 -9.99804,3.4668 -3.96919,-0.31243 -8.69755,-2.0252 -13.69727,-4.47656 -9.99943,-4.90273 -20.88848,-12.61967 -31.03711,-16.16016 -9.40334,-3.28048 -19.11621,-3.99575 -28.84375,-3.67773 z",
        "M93.236328,154.05859 c -3.682182,-0.001 -7.367143,0.58118 -10.96289,1.36524 -7.191495,1.56812 -14.218755,3.95004 -20.587891,4.78515 l 0.78125,5.94922 c 7.22512,-0.94734 14.364494,-3.40742 21.085937,-4.87304 6.721444,-1.46563 12.831095,-1.92708 18.244146,0.41796 1.31363,0.5691 2.40433,1.74556 3.45117,3.62305 1.04683,1.87749 1.94737,4.3451 2.90625,6.89063 0.95887,2.54552 1.9636,5.17601 3.5664,7.48632 1.60281,2.31032 4.0819,4.35727 7.32813,4.79493 18.32191,2.47018 36.70066,1.43401 54.92578,-0.34375 l -0.58203,-5.97071 c -18.05916,1.76158 -35.97008,2.73834 -53.54297,0.36914 -1.36836,-0.18448 -2.18586,-0.80885 -3.19922,-2.26953 -1.01336,-1.46068 -1.9467,-3.70174 -2.88086,-6.18164 -0.93415,-2.4799 -1.8829,-5.18933 -3.28125,-7.69726 -1.39835,-2.50793 -3.34766,-4.92514 -6.30664,-6.20703 -3.58386,-1.55261 -7.26313,-2.13736 -10.945312,-2.13868 z m 126.199222,20.01953 c -2.82755,-0.0831 -5.67453,-0.0225 -8.54102,0.22461 l 0.51563,5.97852 c 20.80879,-1.79414 42.25766,8.00586 65.10937,8.00586 v -6 c -18.41989,0 -37.29117,-7.62698 -57.08398,-8.20899 z",
        "M93.806641,183.77734 c -11.315126,-0.13224 -22.479199,2.98694 -32.875,3.68946 l 0.404297,5.98632 c 14.959167,-1.01089 29.473874,-5.75564 42.312502,-2.4414 l 1.5,-5.8086 c -3.78262,-0.97646 -7.570091,-1.38169 -11.341799,-1.42578 z m 43.849609,12.16602 -0.67187,5.96094 c 22.6526,2.55729 45.40177,2.42578 66.89843,7.46484 11.72513,2.74851 21.48797,5.87483 29.72657,6.19141 7.52568,0.28916 14.95175,-1.70331 21.84765,-2.86328 6.8959,-1.15998 13.04955,-1.49331 18.30859,1.13867 l 2.68555,-5.36524 c -7.08387,-3.54524 -14.72832,-2.91099 -21.99023,-1.68945 -7.26192,1.22154 -14.35456,3.02399 -20.6211,2.7832 -6.82274,-0.26216 -16.62775,-3.23351 -28.58789,-6.03711 -22.42372,-5.25637 -45.42207,-5.08076 -67.5957,-7.58398 z",
        "M101.34375,222.14453 c -11.915491,-0.38005 -23.809414,0.33729 -35.582031,2.51758 l 1.091797,5.90039 c 45.161694,-8.36396 93.271904,5.7762 137.707034,20.58594 l 1.89648,-5.69141 c -33.42928,-11.14161 -69.36681,-22.17235 -105.11328,-23.3125 z m 140.42969,10.91406 c -3.65876,-0.21192 -7.35603,-0.24909 -11.0918,0.0273 l 0.44336,5.98242 c 13.57108,-1.00421 27.7339,2.83008 42.57227,2.83008 v -6 c -10.3153,0 -20.94756,-2.20408 -31.92383,-2.83985 z",
        "M135.9082,249.06641 c -6.63407,0 -12.00903,3.43897 -16.78711,7.08007 -4.77807,3.64111 -9.16867,7.58585 -13.63281,9.65821 -6.384119,2.96364 -15.125811,4.48387 -23.103514,3.75 -7.977704,-0.73388 -15.011192,-3.66727 -18.826172,-8.90821 l -4.84961,3.53125 c 5.213707,7.16248 14.032238,10.51493 23.126954,11.35157 9.094715,0.83663 18.646612,-0.7871 26.177732,-4.28321 5.65903,-2.62704 10.26447,-6.91246 14.74414,-10.32617 4.47968,-3.41371 8.62368,-5.85351 13.15039,-5.85351 z m 27.89258,4.90429 -2.17383,5.5918 c 0.64279,0.24994 1.23113,0.84192 1.88086,2.14844 0.64974,1.30651 1.21441,3.16606 1.81641,5.125 0.602,1.95894 1.22072,4.02182 2.39844,5.91992 1.17771,1.8981 3.32051,3.72029 6.04296,3.93359 33.00536,2.58613 66.99569,0.93388 99.24805,-7.13672 l -1.45508,-5.82031 c -31.47542,7.87618 -64.83765,9.52009 -97.32421,6.97461 -0.6586,-0.0516 -0.82751,-0.17109 -1.41211,-1.11328 -0.58461,-0.94219 -1.17963,-2.61903 -1.76368,-4.51953 -0.58404,-1.9005 -1.17719,-4.01931 -2.17968,-6.03516 -1.0025,-2.01585 -2.56005,-4.08925 -5.07813,-5.06836 z",
    ]

    def __init__(self, bomb, ident):
        super().__init__(bomb, ident)
        wire_count = random.randint(3, 8)
        if wire_count > 6: wire_count = 6
        self.positions = sorted(random.sample(range(6), wire_count))
        self.cut = [False] * wire_count
        self.colors = []
        for _ in range(wire_count):
            self.colors.append(random.choice(list(Wires.Color)))
        self.log(f"There are {len(self.colors)} wires: {' '.join(color.name for color in self.colors)}")

    def get_svg(self, led):
        svg = (
            f'<svg viewBox="0 0 348 348" fill="#fff" stroke="none" stroke-width="2" stroke-linecap="butt" stroke-linejoin="round" stroke-miterlimit="10">'
            f'<path stroke="#000" d="M5 5h338v338h-338zM47 62h30v226h-30zM258 107h30v178h-30z"/>'
            f'<circle fill="{led}" stroke="#000" cx="298" cy="40.5" r="15" stroke-width="2"/>')
        for pos, color, cut in zip(self.positions, self.colors, self.cut):
            paths = Wires.PATHS_CUT if cut else Wires.PATHS_UNCUT
            svg += f'<path fill="{color.value}" stroke="#000" d="{paths[pos]}" />'
        svg += '</svg>'
        return svg

    @modules.check_solve_cmd
    async def cmd_cut(self, author, parts):
        if len(parts) != 1 or not parts[0].isdigit():
            await self.usage(author)
        elif parts[0] == "0":
            await self.bomb.channel.send(f"{author.mention} Arrays start at 0, but wires start at 1.")
        else:
            wire = int(parts[0]) - 1
            if wire not in range(len(self.colors)):
                await self.bomb.channel.send(f"There are only {len(self.colors)} wires. How on earth am I supposed to cut wire {parts[0]}?")
            else:
                expected = self.get_solution()
                self.log(f"player cut wire {wire+1}. expected wire {expected+1}")
                self.cut[wire] = True
                if expected == wire:
                    await self.handle_solve(author)
                else:
                    await self.handle_strike(author)

    def get_solution(self):
        def count(color):
            return self.colors.count(color)

        def first(color):
            return self.colors.index(color)

        def last(color):
            return len(self.colors) - 1 - self.colors[::-1].index(color)

        serial_odd = int(self.bomb.serial[-1]) % 2 == 1
        self.log('the last digit of the serial number is {:s}'.format('odd' if serial_odd else 'even'))

        if len(self.colors) == 3:
            if count(Wires.Color.red) == 0:
                self.log('rule: there are no red wires')
                return 1
            elif self.colors[-1] == Wires.Color.white:
                self.log('rule: the last wire is white')
                return 2
            elif count(Wires.Color.blue) > 1:
                self.log('rule: there is more than one blue wire')
                return last(Wires.Color.blue)
            else:
                self.log('rule: wildcard')
                return 2
        elif len(self.colors) == 4:
            if count(Wires.Color.red) > 1 and serial_odd:
                self.log('rule: there is more than one red wire')
                return last(Wires.Color.red)
            elif self.colors[-1] == Wires.Color.yellow and count(Wires.Color.red) == 0:
                self.log('rule: the last wire is yellow and there are no red wires')
                return 0
            elif count(Wires.Color.blue) == 1:
                self.log('rule: there is exactly one blue wire')
                return 0
            elif count(Wires.Color.yellow) > 1:
                self.log('rule: there is more than one yellow wire')
                return 3
            else:
                self.log('rule: wildcard')
                return 1
        elif len(self.colors) == 5:
            if self.colors[-1] == Wires.Color.black and serial_odd:
                self.log('rule: the last wire is black and the last digit of the serial number is odd')
                return 3
            elif count(Wires.Color.red) == 1 and count(Wires.Color.yellow) > 1:
                self.log('rule: there is exactly one red wire and there is more than one yellow wire')
                return 0
            elif count(Wires.Color.black) == 0:
                self.log('rule: there are no black wires')
                return 1
            else:
                self.log('rule: wildcard')
                return 0
        else:
            if count(Wires.Color.yellow) == 0 and serial_odd:
                self.log('rule: there are no yellow wires and the last digit of the serial number is odd')
                return 2
            elif count(Wires.Color.yellow) == 1 and count(Wires.Color.white) > 1:
                self.log('rule: there is exactly one yellow wire and there is more than one white wire')
                return 3
            elif count(Wires.Color.red) == 0:
                self.log('rule: there are no red wires')
                return 5
            else:
                self.log('rule: wildcard')
                return 3

    COMMANDS = {
        "cut": cmd_cut
    }

class ComplicatedWires(modules.Module):
    display_name = "Complicated Wires"
    manual_name = "Complicated Wires"
    help_text = "`{cmd} cut 3` - cut the third wire. `{cmd} cut 1 4 6` - cut multiple wires. `{cmd} cut 146` - cut multiple wires, shorter. Wires are counted left to right, empty spaces excluded."
    module_score = 3

    @enum.unique
    class Color(enum.Enum):
        blue = "#00f"
        red = "#f00"
        white = "#fff"

    PATHS_UNCUT = [
        "m55.183594,73.0625 -5.96875,0.597656 c 1.652446,16.509105 -6.128906,33.805904 -6.128906,52.371094 0,21.07678 0.568469,43.28739 8.185546,63.59961 1.331273,3.55006 3.701096,6.19525 5.539063,8.61523 1.837967,2.41999 3.074391,4.46932 3.210937,6.51953 1.225156,18.39563 -10.351562,37.1942 -10.351562,58.11719 h 6 c 0,-18.56083 11.738513,-37.5147 10.339844,-58.51562 -0.269181,-4.04172 -2.428071,-7.1274 -4.419922,-9.75 -1.991852,-2.6226 -3.886727,-4.92711 -4.699219,-7.09375 -7.142932,-19.04783 -7.804688,-40.53241 -7.804687,-61.49219 0,-16.78337 7.965763,-34.305048 6.097656,-52.96875z",
        "m86.230469,69.636719 -2.226563,5.572265 c 3.834857,1.533657 5.377102,3.856568 6.123047,7.611328 0.745945,3.754761 0.261574,8.89871 -0.939453,14.574219 -2.402055,11.351019 -7.5128,24.593059 -8.46875,35.427739 -1.366517,15.48802 3.902182,30.41984 8.496094,44.86914 4.593911,14.44929 8.507042,28.34377 5.666015,41.77343 -0.940575,4.44615 -4.042214,8.73833 -7.333984,13.39063 -3.29177,4.6523 -6.807863,9.74342 -7.300781,16.1582 -0.320857,4.1756 2.248322,7.48703 5.09375,9.77344 2.845428,2.28641 6.224862,3.88454 9.011718,4.99805 l 2.22461,-5.57032 c -2.463196,-0.98418 -5.413354,-2.44564 -7.476563,-4.10351 -2.063208,-1.65787 -2.983388,-3.17532 -2.871093,-4.63672 0.339593,-4.41943 2.990216,-8.59218 6.216796,-13.15234 3.226581,-4.56017 6.996316,-9.4305 8.304688,-15.61524 3.23945,-15.31301 -1.190873,-30.28523 -5.816406,-44.83398 -4.625533,-14.54876 -9.451889,-28.74634 -8.236328,-42.52344 0.821767,-9.31386 5.805985,-22.64673 8.359375,-34.712891 1.276695,-6.03308 1.985265,-11.808695 0.957031,-16.984375 -1.028234,-5.175681 -4.227759,-9.793862 -9.783203,-12.015625z",
        "m120.60938,67.082031 0.49804,5.978516 c 4.26587,-0.354994 6.40225,1.129047 8.36719,4.525391 1.96493,3.396343 3.1526,8.876021 3.56445,15.197265 0.8237,12.642487 -1.21262,28.488657 -2.46094,39.011717 -1.40189,11.81775 4.62463,22.15509 10.41407,31.92578 5.78944,9.7707 11.36663,19.07419 11.19726,28.56836 -0.20773,11.64469 -5.69976,25.40505 -9.36133,38.82227 -1.83078,6.70861 -3.1962,13.3675 -3.01367,19.81445 0.18254,6.44695 2.01555,12.74309 6.48828,18.10938 l 4.60743,-3.8418 c -3.53694,-4.24353 -4.94291,-8.97173 -5.09766,-14.4375 -0.15475,-5.46577 1.04317,-11.60964 2.80469,-18.06445 3.52304,-12.90963 9.33372,-26.92526 9.57226,-40.29688 0.21093,-11.82354 -6.28534,-22.0286 -12.03515,-31.73242 -5.74982,-9.70382 -10.72888,-18.80524 -9.61914,-28.16016 1.23969,-10.45041 3.37762,-26.51929 2.49218,-40.109372 -0.44272,-6.795043 -1.58142,-13.010882 -4.35937,-17.8125 -2.77795,-4.801617 -7.80058,-8.01882 -14.05859,-7.498047z",
        "m155.36328,72.589844 -4.11133,4.36914 c 11.75385,11.059264 11.30106,31.809846 14.14063,50.007816 3.96137,25.3871 11.7207,50.0124 11.7207,74.77929 0,21.43636 -7.92455,46.43383 2.66797,67.64844 l 5.36719,-2.67969 c -9.16758,-18.36074 -2.03516,-42.17085 -2.03516,-64.96875 0,-26.00263 -7.92704,-50.92964 -11.79297,-75.70507 -2.69772,-17.28892 -1.5531,-39.898426 -15.95703,-53.451176z",
        "m196.48828,76.369141 -5.47656,2.453125 c 18.178,40.596954 36.05265,82.458164 41.14258,126.091794 2.35171,20.16027 -2.8418,41.14157 -2.8418,62.66797 h 6 c 0,-20.52618 5.3215,-41.75416 2.80078,-63.36328 -5.22559,-44.79666 -23.41918,-87.19053 -41.625,-127.849609z",
        "m218.76758,63.625 -4.88281,3.486328 c 11.34339,15.885821 27.56497,27.220753 36.00781,42.941402 14.39234,26.79868 7.50226,61.57603 13.80664,93.6836 4.3622,22.21613 17.87964,42.47321 15.02344,62.48242 l 5.93945,0.84766 c 3.2904,-23.051 -11.0577,-44.02079 -15.07617,-64.48633 C 263.61425,172.16687 270.9312,136.546 255.17773,107.21289 245.8248,89.79762 229.28124,78.348813 218.76758,63.625z",
    ]

    PATHS_CUT = [
        "m55.183594,73.0625 -5.96875,0.597656 c 1.652447,16.509104 -6.128907,33.805904 -6.128906,52.371094 0,12.99041 0.228254,26.40399 2.173828,39.54687 l 5.935546,-0.8789 c -1.868566,-12.62268 -2.109375,-25.74966 -2.109374,-38.66797 0,-16.78337 7.965763,-34.305048 6.097656,-52.96875 z m 1.707031,114.46094 -5.619141,2.10742 c 1.331273,3.55006 3.701096,6.19525 5.539063,8.61523 1.837967,2.41999 3.074391,4.46931 3.210937,6.51953 1.225156,18.39562 -10.351562,37.1942 -10.351562,58.11719 h 6 c 0,-18.56083 11.738514,-37.5147 10.339844,-58.51562 -0.269181,-4.04172 -2.428071,-7.1274 -4.419922,-9.75 -1.991852,-2.6226 -3.886728,-4.92711 -4.699219,-7.09375z",
        "m86.230469,69.636719 -2.226563,5.572265 c 2.581708,1.032491 4.028306,2.441833 5.029297,4.371094 1.000991,1.929261 1.483357,4.500197 1.458985,7.607422 -0.04874,6.214449 -2.16266,14.34445 -4.478516,22.55078 l 5.77539,1.63086 c 2.330352,-8.25769 4.644524,-16.663619 4.703126,-24.134765 0.0293,-3.735573 -0.513014,-7.296047 -2.132813,-10.417969 -1.619799,-3.121922 -4.388862,-5.683948 -8.128906,-7.179687 z M 80.71875,132.82227 c -1.366518,15.48802 3.902182,30.41984 8.496094,44.86914 4.593912,14.44929 8.507042,28.34377 5.666015,41.77343 -0.940575,4.44615 -4.042214,8.73833 -7.333984,13.39063 -3.29177,4.6523 -6.807863,9.74342 -7.300781,16.1582 -0.320856,4.1756 2.248322,7.48703 5.09375,9.77344 2.845427,2.28641 6.224863,3.88454 9.011718,4.99805 l 2.22461,-5.57032 c -2.463198,-0.98418 -5.413354,-2.44564 -7.476563,-4.10351 -2.063208,-1.65787 -2.983388,-3.17533 -2.871093,-4.63672 0.339593,-4.41944 2.990216,-8.59218 6.216796,-13.15234 3.226581,-4.56017 6.996316,-9.4305 8.304688,-15.61524 3.23945,-15.31302 -1.190873,-30.28523 -5.816406,-44.83398 -4.625533,-14.54876 -9.45189,-28.74634 -8.236328,-42.52344z",
        "m120.60938,67.082031 0.49804,5.978516 c 4.26587,-0.354994 6.40225,1.129048 8.36719,4.525391 1.96493,3.396342 3.1526,8.876021 3.56445,15.197265 0.8237,12.642487 -1.21263,28.488657 -2.46094,39.011717 -1.16475,9.81877 2.85207,18.65557 7.50782,26.92383 l 5.22851,-2.94336 c -4.47317,-7.94402 -7.70132,-15.50091 -6.77929,-23.27344 1.23969,-10.45041 3.37762,-26.51929 2.49218,-40.109372 -0.44272,-6.795043 -1.58142,-13.010883 -4.35937,-17.8125 -2.77795,-4.801617 -7.80058,-8.01882 -14.05859,-7.498047 z m 31.58007,125.207029 c -0.20773,11.6447 -5.69976,25.40505 -9.36133,38.82227 -1.83078,6.70861 -3.1962,13.3675 -3.01367,19.81445 0.18254,6.44695 2.01555,12.74309 6.48828,18.10938 l 4.60743,-3.8418 c -3.53694,-4.24353 -4.94291,-8.97173 -5.09766,-14.4375 -0.15475,-5.46577 1.04317,-11.60964 2.80469,-18.06445 3.52304,-12.90962 9.33372,-26.92525 9.57226,-40.29688z",
        "m155.36328,72.589844 -4.11133,4.36914 c 6.80114,6.399235 9.61633,15.891157 11.23438,26.468746 l 5.93164,-0.9082 C 166.72604,91.458969 163.69784,80.431887 155.36328,72.589844 Z m 15.95703,53.451176 -5.92773,0.92578 c 3.96137,25.3871 11.7207,50.01239 11.7207,74.77929 0,21.43636 -7.92455,46.43383 2.66797,67.64844 l 5.36719,-2.67969 c -9.16758,-18.36075 -2.03516,-42.17085 -2.03516,-64.96875 0,-26.00264 -7.92704,-50.92964 -11.79297,-75.70507z",
        "m196.48828,76.369141 -5.47656,2.453125 c 13.37913,29.879634 26.6061,60.447194 34.67383,91.898434 l 5.81054,-1.49023 C 223.2691,137.15833 209.88789,106.2945 196.48828,76.369141 Z m 41.625,127.849609 -5.95898,0.69531 c 2.35172,20.16026 -2.8418,41.14157 -2.8418,62.66797 h 6 c 0,-20.52618 5.32152,-41.75416 2.80078,-63.36328z",
        "m218.76758,63.625 -4.88281,3.486328 c 11.3434,15.88582 27.56496,27.220758 36.00781,42.941402 9.06802,16.88477 9.83522,36.88254 10.58398,57.46485 0.23517,6.46442 0.47623,12.97698 1.00391,19.45508 l 5.98047,-0.48828 c -0.51322,-6.30058 -0.75323,-12.72435 -0.98828,-19.18555 -0.7484,-20.57205 -1.36931,-41.60433 -11.29493,-60.08594 C 245.82478,89.797629 229.28124,78.348814 218.76758,63.625 Z m 50.81836,138.95508 -5.88672,1.15625 c 4.36221,22.21613 17.87964,42.47321 15.02344,62.48242 l 5.93945,0.84766 c 3.2904,-23.051 -11.0577,-44.02079 -15.07617,-64.48633z",
    ]

    def condB(self):
        return self.bomb.get_battery_count() >= 2

    def condC(self):
        return True

    def condD(self):
        return False

    def condP(self):
        return self.bomb.has_port(edgework.PortType.Parallel)

    def condS(self):
        return int(self.bomb.serial[-1]) % 2 == 0

    RULES = {
        # red blue led star
        (False, False, False, False): condC,
        (False, False, False, True):  condC,
        (False, False, True, False):  condD,
        (False, False, True, True):   condB,
        (False, True, False, False):  condS,
        (False, True, False, True):   condD,
        (False, True, True, False):   condP,
        (False, True, True, True):    condP,
        (True, False, False, False):  condS,
        (True, False, False, True):   condC,
        (True, False, True, False):   condB,
        (True, False, True, True):    condB,
        (True, True, False, False):   condS,
        (True, True, False, True):    condP,
        (True, True, True, False):    condS,
        (True, True, True, True):     condD,
    }

    def __init__(self, bomb, ident):
        super().__init__(bomb, ident)
        wire_count = random.randint(4, 9)
        if wire_count > 6: wire_count = 6
        self.positions = sorted(random.sample(range(6), wire_count))
        self.cut = [False] * wire_count
        self.should_cut = [False] * wire_count

        colorings = list(itertools.chain(ComplicatedWires.Color, itertools.combinations(ComplicatedWires.Color, 2)))
        self.wire_colors = []
        self.leds = []
        self.stars = []

        cut_combinations = self.get_cut_combinations()
        encountered_cut = False
        for index in range(wire_count):
            self.wire_colors.append(random.choice(colorings))
            self.leds.append(random.random() > 0.5)
            self.stars.append(random.random() > 0.5)
            if self.wire_to_rules(index) in cut_combinations:
                encountered_cut = True
                self.should_cut[index] = True
            self.log(f'Adding wire: {self.wire_to_string(-1)} - {"should cut" if self.should_cut[index] else "should NOT cut"}')

        if not encountered_cut:
            self.log('No wires to cut, overwriting a random wire...')
            index = random.randint(0, wire_count - 1)
            self.set_wire_rules(index, random.choice(cut_combinations))
            self.should_cut[index] = True

    def wire_to_string(self, index):
        color = self.wire_colors[index]
        if isinstance(color, tuple):
            color = f'{color[0].name}-{color[1].name}'
        else:
            color = color.name
        return f'{color}, {"LED" if self.leds[index] else "no LED"}, {"star" if self.stars[index] else "no star"}'

    def wire_to_rules(self, index):
        colors = self.wire_colors[index]
        if isinstance(colors, tuple):
            red = ComplicatedWires.Color.red in colors
            blue = ComplicatedWires.Color.blue in colors
        else:
            red = colors == ComplicatedWires.Color.red
            blue = colors == ComplicatedWires.Color.blue
        return (red, blue, self.leds[index], self.stars[index])

    def set_wire_rules(self, index, rules):
        red, blue, led, star = rules
        if red and blue:
            self.wire_colors[index] = (ComplicatedWires.Color.red, ComplicatedWires.Color.blue)
        elif red:
            self.wire_colors[index] = ComplicatedWires.Color.red
        elif blue:
            self.wire_colors[index] = ComplicatedWires.Color.blue
        else:
            self.wire_colors[index] = ComplicatedWires.Color.white
        self.leds[index] = led
        self.stars[index] = star
        self.log(f'Overwrote wire {index + 1} with {self.wire_to_string(index)}')

    def get_cut_combinations(self):
        combinations = []

        for combination, condition in ComplicatedWires.RULES.items():
            if condition(self):
                combinations.append(combination)

        return combinations

    def get_svg(self, led):
        needed_gradients = {coloring for coloring in self.wire_colors if isinstance(coloring, tuple)}

        svg = '<svg viewBox="0 0 348 348" fill="#fff" stroke="none" stroke-linecap="butt" stroke-linejoin="round" stroke-miterlimit="10">'
        if needed_gradients:
            svg += '<defs>'
            for gradient in needed_gradients:
                svg += f'<linearGradient id="{gradient[0].name}-{gradient[1].name}" x1="0%" x2="5%" y1="0%" y2="100%">'
                for percent in range(0, 100, 20):
                    svg += (f'<stop offset="{percent}%" stop-color="{gradient[0].value}"/>'
                        f'<stop offset="{percent + 10}%" stop-color="{gradient[0].value}"/>'
                        f'<stop offset="{percent + 10}%" stop-color="{gradient[1].value}"/>'
                        f'<stop offset="{percent + 20}%" stop-color="{gradient[1].value}"/>')
                svg += '</linearGradient>'
            svg += '</defs>'
        svg += ('<path stroke="#000" stroke-width="2" d="M5 5h338v388h-338z"/>'
            '<path stroke="#000" stroke-width="2" fill="#888" d="M29 29v58h224v-58zM29 250h274v74h-274z"/>'
            '<path stroke="#000" stroke-width="2" d="M29 58h224M29 279h274M39 284h29l5 5v29h-29l-5 -5zM83 284h29l5 5v29h-29l-5 -5zM127 284h29l5 5v29h-29l-5 -5zM171 284h29l5 5v29h-29l-5 -5zM215 284h29l5 5v29h-29l-5 -5zM259 284h29l5 5v29h-29l-5 -5z"/>'
            f'<circle fill="{led}" stroke="#000" cx="298" cy="40.5" r="15" stroke-width="2"/>')

        for position in range(6):
            if position in self.positions:
                color = "#fec" if self.leds[self.positions.index(position)] else "#444"
            else:
                color = "#444"
            svg += f'<circle fill="{color}" r="8.5" cx="{50 + position * 35}" cy="43" stroke="#000" stroke-width="2"/>'

        for color, star, cut, position in zip(self.wire_colors, self.stars, self.cut, self.positions):
            path = (ComplicatedWires.PATHS_CUT if cut else ComplicatedWires.PATHS_UNCUT)[position]
            if isinstance(color, tuple):
                color_str = f'url(#{color[0].name}-{color[1].name})'
            else:
                color_str = color.value

            svg += f'<path stroke="#000" stroke-width="2" fill="{color_str}" d="{path}"/>'
            if star:
                svg += f'<path fill="#000" d="M{39 + position * 44} 284m6.5 15h8l2.5-8l2.5 8h8l-6.5 4.5l2.5 7.5l-6.5 -4.5l-6.5 4.5l2.5-7.5z"/>'
        svg += '</svg>'
        return svg

    @modules.check_solve_cmd
    async def cmd_cut(self, author, parts):
        if not parts:
            return await self.usage(author)

        parsed = []

        for part in parts:
            if not part.isdigit():
                return await self.usage(author)
            for digit in part:
                wire = int(digit) - 1
                if wire not in range(len(self.wire_colors)):
                    return await self.bomb.channel.send(f"{author.mention} There are only {len(self.wire_colors)} wires: 1-{len(self.wire_colors)}")
                parsed.append(wire)

        self.log(f'Parsed: {" ".join(map(str, parsed))}')

        for wire in parsed:
            if self.cut[wire]:
                self.log(f'Wire {wire + 1} has already been cut, ignoring')
            else:
                self.cut[wire] = True
                if self.should_cut[wire]:
                    if self.is_everything_done():
                        return await self.handle_solve(author)
                else:
                    return await self.handle_strike(author)

        await self.do_view(f"{author.mention} You seem to have missed some wire:")

    def is_everything_done(self):
        for cut, should_cut in zip(self.cut, self.should_cut):
            if should_cut and not cut:
                return False
        return True

    COMMANDS = {
        'cut': cmd_cut,
    }

class WireSequence(modules.Module):
    display_name = "Wire Sequence"
    manual_name = "Wire Sequence"
    help_text = "`{cmd} cut 7` - cut wire 7. `{cmd} down`, `{cmd} d` - go to the next panel. `{cmd} up`, `{cmd} u` - go back to the previous panel. `{cmd} cut 1 3 d` - cut mutiple wires and continue."
    module_score = 4

    @enum.unique
    class Color(enum.Enum):
        red = "#f00"
        blue = "#00f"
        black = "#000"

    RULES = {
        Color.red: [{2}, {1}, {0}, {0, 2}, {1}, {0, 2}, {0, 1, 2}, {0, 1}, {1}],
        Color.blue: [{1}, {0, 2}, {1}, {0}, {1}, {1, 2}, {2}, {0, 2}, {0}],
        Color.black: [{0, 1, 2}, {0, 2}, {1}, {0, 2}, {1}, {1, 2}, {0, 1}, {2}, {2}],
    }

    PATHS_UNCUT = {
        (0, 0): "m 125.13854,98.813083 -0.0102,5.987437 110.90849,0.18126 0.0102,-5.987438 z",
        (0, 1): "m 127.00048,97.767359 -3.47859,5.083901 109.64552,72.63119 3.48062,-5.08591 z",
        (0, 2): "m 129.53502,99.238202 -4.65119,3.578928 105.32434,140.16172 4.6512,-3.58087 z",

        (1, 0): "m 127.22803,174.78975 -3.49156,-5.08365 110.05422,-72.62761 3.4936,5.08566 z",
        (1, 1): "m 125.05355,168.88342 -0.0102,5.98744 110.90849,0.18126 0.0102,-5.98744 z",
        (1, 2): "m 126.70868,167.63502 -3.49156,5.08365 110.05423,72.62761 3.4936,-5.08566 z",

        (2, 0): "m 233.39689,100.10766 -108.46288,139.99505 4.8193,3.70926 108.46287,-139.99506 z",
        (2, 1): "m 126.37656,245.93829 -3.49156,-5.08365 110.05423,-72.62761 3.4936,5.08566 z",
        (2, 2): "m 126.15868,237.19896 -0.01,6 108.79297,0.18164 0.01,-6 z",
    }

    PATHS_CUT = {
        (0, 0): "m 170.063,98.899922 -44.92446,-0.08649 v -3.49e-4 l -0.0102,5.987437 44.93466,0.19343 z m 20.278,6.100858 45.69583,-0.0186 0.0102,-5.987438 -45.70603,-0.08903 z",
        (0, 1): "m 185.50472,143.90982 47.66269,31.57263 3.48062,-5.08591 -47.6446,-31.43119 z m -10.23625,-14.07064 -48.26799,-32.071821 -3.47859,5.083901 48.37564,32.04491 z",
        (0, 2): "m 184.97163,182.77974 45.23654,60.19911 4.6512,-3.58087 -45.20169,-60.22656 z m -7.82191,-20.22949 -47.6147,-63.312048 -4.65119,3.578928 47.57337,63.30888 z",

        (1, 0): "m 175.08766,142.95297 -47.85963,31.83678 -3.49156,-5.08365 47.79483,-31.54104 z m 10.44371,-13.92758 48.25932,-31.9469 3.4936,5.08566 -48.13157,31.75874 z",
        (1, 1): "m 169.86073,168.95672 -44.80718,-0.0733 -0.0102,5.98744 44.7423,0.0724 z m 24.98627,6.02774 41.10484,0.0677 0.0102,-5.98744 -41.11497,-0.0672 z",
        (1, 2): "m 186.1344,214.2394 47.13695,31.10688 3.4936,-5.08566 -47.40858,-31.0675 z m -10.96829,-14.62756 -48.45743,-31.97682 -3.49156,5.08365 48.65914,32.03903 z",

        (2, 0): "m 189.62979,166.397 48.58639,-62.58009 -4.81929,-3.70925 -48.58051,62.70377 z m -17.70146,13.04927 -46.99432,60.65644 4.8193,3.70926 47.07207,-60.58177 z",
        (2, 1): "m 175.99116,213.22973 -49.6146,32.70856 -3.49156,-5.08365 49.669,-32.77784 z m 8.85538,-13.2651 48.09269,-31.7376 3.4936,5.08566 -48.16212,31.88838 z",
        (2, 2): "m 186.5774,243.29985 48.36425,0.0807 0.01,-6 -48.37365,-0.0809 z m -13.41046,-6.02241 -47.00826,-0.0785 -0.01,6 47.01838,0.18422 z",
    }

    def __init__(self, bomb, ident):
        super().__init__(bomb, ident)
        self.current_page = 0
        self.solved_pages = 0

        # This is how the game generates these wires. Please don't ask me about this code.
        self.wires = []
        for color in WireSequence.Color:
            for _ in range(9):
                self.wires.append((color, random.randint(0, 2)))
        self.wires = random.sample(self.wires, 10) # replicating an off-by-one error
        self.wires += [None] * 3
        random.shuffle(self.wires)
        # if the first page is empty, all the other pages will be full
        if self.wires[:3].count(None) == 3:
            self.wires[random.randint(0, 2)] = self.wires[3]
            self.wires[3] = None

        self.wires.pop() # remove the additional wire caused by the off-by-one error

        self.should_cut = [False] * 12
        self.cut = [False] * 12
        counts = {}
        for index, wire in enumerate(self.wires):
            if wire is None: continue
            color, to = wire
            if color not in counts:
                counts[color] = 0
            self.should_cut[index] = to in WireSequence.RULES[color][counts[color]]
            counts[color] += 1

            should_cut = "cut" if self.should_cut[index] else "don't count"
            self.log(f"Wire {index + 1} to {'ABC'[to]} is the {counts[color]}. {color.name} wire - {should_cut}")

    def get_svg(self, led):
        svg = (
            f'<svg viewBox="0 0 348 348" fill="#fff" stroke-width="2" stroke-linejoin="round" stroke-linecap="butt" stroke-miterlimit="10" xmlns="http://www.w3.org/2000/svg">'
            f'<path stroke="#000" d="M5 5h338v338h-338z"/>'
            f'<path stroke="#000" d="M74 74h200v200h-200zM129 19h90v40h-90zM129 288h90v40h-90z"/>'
            f'<circle fill="{led}" stroke="#000" cx="298" cy="40.5" r="15"/>'
            f'<path fill="#000" d="M158 39l16-10 16 10h-8v10h-16v-10zM158 308l16 10 16-10h-8v-10h-16v10z"/>'
            f'<path fill="#000" stroke="#000" d="M283 74h52v254h-52z"/>')

        for i in range(4):
            color = "#0f0" if self.solved_pages > i else "#fff"
            svg += f'<path fill="{color}" stroke="{color}" d="M294 {273 - 55 * i}h30v21h-30z"/>'

        if self.solved:
            svg += f'<path stroke="#000" d="M74 174h200"/>'
        else:
            for i in range(3):
                wire_index = self.current_page * 3 + i

                svg += (
                    f'<text x="104" y="{114 + 70 * i}" text-anchor="middle" fill="#000" style="font-family:sans-serif;font-size:24pt;">{wire_index + 1}</text>'
                    f'<text x="249" y="{114 + 70 * i}" text-anchor="middle" fill="#000" style="font-family:sans-serif;font-size:24pt;">{"ABC"[i]}</text>')

                wire = self.wires[wire_index]
                if wire is not None:
                    color, to = wire
                    path = WireSequence.PATHS_CUT[i, to] if self.cut[wire_index] else WireSequence.PATHS_UNCUT[i, to]
                    svg += f'<path fill="{color.value}" stroke="#000" d="{path}"/>'

        svg += f'</svg>'
        return svg

    @modules.check_solve_cmd
    @modules.noparts
    async def cmd_up(self, author):
        if self.current_page == 0:
            return await self.do_view(f"{author.mention} This is the first panel already!")
        self.current_page -= 1
        return await self.do_view(author.mention)

    @modules.check_solve_cmd
    @modules.noparts
    async def cmd_down(self, author):
        return await self.do_down(author)

    async def do_down(self, author):
        begin = 3 * self.current_page
        end = begin + 3
        page_should_cut = self.should_cut[begin:end]
        page_cut = self.cut[begin:end]
        for index, should_cut, cut in zip(range(begin, end), page_should_cut, page_cut):
            if should_cut and not cut:
                self.log(f"Didn't cut wire {index + 1}!")
                return await self.handle_strike(author)

        self.current_page += 1
        if self.solved_pages < self.current_page:
            self.solved_pages = self.current_page

        if self.current_page >= 4:
            return await self.handle_solve(author)
        else:
            return await self.do_view(author.mention)

    @modules.check_solve_cmd
    async def cmd_cut(self, author, parts):
        wires_to_cut = []
        go_to_next_page = False
        for part in parts:
            if go_to_next_page:
                return await self.bomb.channel.send(f"{author.mention} Trailing arguments")

            if part.isdigit():
                wire = int(part)

                if wire not in range(1, 12 + 1):
                    return await self.do_view(f"{author.mention} Wires are numbered 1-12. What on earth does wire {wire} mean?")

                wire -= 1
                first_on_page = 3 * self.current_page
                end_range = first_on_page + 3

                if wire not in range(first_on_page, end_range):
                    return await self.do_view(f"{author.mention} Wire {wire + 1} is not on this panel. The wires on this panel are numbered {first_on_page + 1}-{end_range}.")

                if self.wires[wire] is None:
                    return await self.do_view(f"{author.mention} There is no wire connected to {wire + 1}!")

                if self.cut[wire]:
                    return await self.do_view(f"{author.mention} Wire {wire + 1} has already been cut!")

                if wire in wires_to_cut:
                    return await self.bomb.channel.send(f"{author.mention} Wait, am I supposed to cut wire {wire + 1} twice?")

                wires_to_cut.append(wire)
            elif part in ['d', 'down']:
                go_to_next_page = True
            else:
                return await self.bomb.channel.send(f"{author.mention} What does `{part}` mean?")

        for wire in wires_to_cut:
            self.log(f"Cutting wire {wire + 1}")
            self.cut[wire] = True
            if not self.should_cut[wire]:
                return await self.handle_strike(author)

        if go_to_next_page:
            return await self.do_down(author)
        else:
            return await self.do_view(author.mention)

    COMMANDS = {
        'cut': cmd_cut,
        'down': cmd_down,
        'd': cmd_down,
        'up': cmd_up,
        'u': cmd_up,
    }
