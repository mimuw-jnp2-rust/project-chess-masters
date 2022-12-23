### Podsumowanie I części projektu



##### Co udało się zrobić? 

Udało nam się zaimplementować szachy, w które da się grać w dwie osoby na jednym komputerze. Gracze wykonują ruchy na zmiane, nie ma możliwości wykonania dwóch ruchów pod rząd przez jednego gracza. Nie ma możliwości wykonania nieprawidłowego ruchu. Jest możliwość wyboru figury (wtedy wybrana figura się podświetla) którą chcemy zrobić ruch. Wtedy także podświetlają się wszystkie możliwe ruchy dla wybranej figury. Da się zbijać figury przeciwnika. 

##### Czego brakuje? 

Jeszcze nie ma obsługi szachowania i matowania, użytkownicy muszą obsłużyć to sami. 
Nie ma roszad i możliwości podmiany piona gdy dojdzie na koniec planszy. 
Aby rozpocząć nową rozgrywkę trzeba od nowa odpalić program. 

##### Plany na kolejną część:

- Menu Startu gry
- Możliwość grania z botem (bot najlepsze ruchy pobierałby z api (https://www.chessdb.cn/cloudbookc_api_en.html)) 
- Dodanie brakujących szczegółów rozgrywki 
- Ewentualne animacje/efekty dźwiękowe i inne upiększacze:) 

##### Tutoriale i inspiracje:

- https://bevyengine.org/examples/ 
  Tutaj przydały się przykłady spawnowania spritów, wyświetlanie FPSów i informacji czyj jest teraz ruch są praktycznie skopiowane 
- https://bevy-cheatbook.github.io/ oraz dokumentacja Bevy
  Cheatbooka i dokumentacji używaliśmy do doczytania o poszczególnych elementach Bevy, których akurat potrzebowaliśmy 
- Gotowe implementacje z szachów w bevy 
  Przeglądaliśmy kilka gotowych implementacji szachów w poszukiwaniu inspiracji. Ostatecznie jednak cała mechanika gry, struktury pomocnicze itp. są w całości napisane przez nas. Nie ma wklejonych fragmentów kodu ani żadnego konkretnego programu,  na którym jakoś szczególnie się wzorowaliśmy.



