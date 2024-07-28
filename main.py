import random
import psutil
import pyautogui
import time
import pydirectinput as pdi
import os.path
import colorama
from pynput.keyboard import Controller
from pyautogui import ImageNotFoundException

pdi.FAILSAFE = False
colorama.init()
keyboard = Controller()
bad_colors = ['BLACK', 'LIGHTBLACK_EX', 'GREY', 'RESET', 'WHITE']
ubisoft_connect, steam, client_id, round_count = "uplay://launch/635", "steam://rungameid/359550", '922380828540026880', 0
cloud_sync_delay = 10

codes = vars(colorama.Fore)
colors = [codes[color] for color in codes if color not in bad_colors]


def checkIfProcessRunning(processName):
    for proc in psutil.process_iter():
        try:
            if processName.lower() in proc.name().lower():
                return True
        except (psutil.NoSuchProcess, psutil.AccessDenied, psutil.ZombieProcess):
            pass
    return False
def locate_onScreen(name, confidence):
    if not confidence > 0 < 1.1:
        return False
    if not os.path.isfile(f"assets\\{name}"):
        return False
    try:
        try:
            pyautogui.locateOnScreen(f'assets\\{name}', confidence=confidence)
            return True
        except ImageNotFoundException:
            time.sleep(1)
            return False
    except IOError:
        print("[!] UAC detected!")
        time.sleep(2)
        return False
    
def press_buttons(button, amount, delay_between, delay_on_the_end):
    for i in range(int(amount)):
        print(f'Pressing {button} ({i + 1}/{amount})')
        pdi.press(str(button))
        if delay_between is not None:
            time.sleep(delay_between)
    if delay_on_the_end is not None:
        time.sleep(delay_on_the_end)
    return True
def press_button(button, delay_on_the_end):
    print(f'Pressing {button}')
    pdi.press(str(button))
    if delay_on_the_end is not None:
        time.sleep(delay_on_the_end)
    return True







class UbisoftGame:
    def __init__(self):
        self.status = None
        self.message = None
        self.tick = 0
        self.valid_buttons = ['w', 'a', 's', 'd'];

        self.enter_the_game()
    def enter_the_game(self):
        self.update_status("starting", "Queueing for first game.");
        
        if self.check_in_queue() == False:
            self.check_queue_game();
            
            time.sleep(5);

        self.update_status("started", "Game queued.");

    def update_status(
        self,
        new_status:  str,
        new_message: str
    ):
        global current_status;
        global tick;

        self.tick += 1;
        self.status = new_status;
        self.message = new_message;
            
        self.display();
    def check_queue_game( self ):
        if locate_onScreen("cogs.png", 0.6):
            self.update_status("queuing", "You are in some sort of menu.");

            press_buttons("up", 3, 0.05, None)
            press_buttons("left", 3, 0.05, None)
            press_button("down", 0.2)
            press_buttons("right", 3, 0.2, 0.2)
            press_buttons("left", 2, 0.2, 0.2)
            press_button("enter", 0.01)
            press_button("right", 0.01)
            press_button("enter", 0.01)
            press_button("enter", 0.01)

            self.update_status("queued", "You are in some sort of menu.");

            return True;
        return False;
    def check_in_queue( self ):
        if locate_onScreen("in_queue.png", 0.8) == True or locate_onScreen("in_queue_alt.png", 0.8) == True:
            self.update_status("in_queue", "You are currently in queue.");
            return True;

        return False;
    def check_game_error( self ):
        if locate_onScreen("in_game_error.png", 0.8):
            press_button("enter", None)

            self.update_status("errored", "Some sort of ingame error occurred.");

            return True;
        return False;
    def game_loop( self ):
        while True:
            if self.check_in_queue():
                time.sleep(5);
                continue;
            if self.check_queue_game():
                time.sleep(30);
                continue;
            if self.check_game_error():
                time.sleep(1.5);
                continue;

            # Lastly, we're probably in game.
            self.update_status("playing", "You are in game.");

            button = random.choice(self.valid_buttons);
            delay_on_the_end = random.uniform(0.2, 0.9);

            press_button(
                button,
                delay_on_the_end
            );

    def display(
        self
    ):
        # Clear the screen
        os.system('cls');

        print( "[ rustfarm ]"                        );
        print( "  property of @hiibolt\n\n"          );
        print( f"<= Status: {self.status.upper()}"   );
        print( f"<= Tick: {self.tick}"               );
        print( f"<= Message: {self.message}\n\n\n"   );
        print( "<= Debug Console:\n")


def startup_with_delay():
    print('\033[39m')
    global start_time
    start_time = time.time()

    print("Waiting, please click on the Rainbow Six window...")
    for i in range(5):
        print(f"{5-i}...")
        time.sleep(1)


def main():
    startup_with_delay()
    game = UbisoftGame()

    while True:
        try:
            game.game_loop()
            time.sleep(5)
        except Exception as e:
            print(e)

if __name__ == "__main__":
    main()