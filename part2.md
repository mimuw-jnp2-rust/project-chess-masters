### Podsumowanie II części projektu

##### Co udało się zrobić?

Udało nam się zaimplementować szachy, w które da się grać w dwie osoby na jednym komputerze, a poza tym jest także możliwość grania przeciwko botowi. Dodaliśmy także bardzo ładne menu początkowe z możliwościami wyboru trybu gry i opcją wyjścia z gry. Grę można zapauzować po naciśnięciu escape. Zaimplementowaliśmy również roszadę i promocję piona (dla uproszczenia zawsze zmienia się w hetmana). Gra wykrywa mata i zawiadamia nas, która strona zwyciężyła. Aby od nowa rozpocząć rozgrywkę mamy opcję wyjścia do menu i ponownie trzeba wybrać tryb. Dodana została muzyka w tle w menu głównym i podczas zapauzowanej gry.

##### Czego brakuje?

Dla uproszczenia pion może zamienić się tylko w hetmana, nie ma bicia w przelocie. Poza tymi szczegółami w zasadzie udało nam się osiągnąć to co zaplanowaliśmy. Komunikacja z botem jest trochę inna niż sobie wyobrażaliśmy, w readme.md jest instrukcja co zrobić by działał. Na początku chcieliśmy pobierać dane z webowego API : https://www.chessdb.cn/queryc_en/ i udało nam się to osiągnąć niestety okazało się, że API jest bardzo słabe i bardzo często nie odsyła żadnego ruchu :( Generalnie nie znaleźliśmy żadnych innych API internetowych, więc nasza aplikacja wymaga pobrania stockfisha.

##### Tutoriale i inspiracje:

- https://bevyengine.org/examples/ - tutaj przydała się głównie obsługa guzików
- https://bevy-cheatbook.github.io/ oraz dokumentacja Bevy Cheatbooka - dokumentacji używaliśmy do doczytania o poszczególnych elementach Bevy, których akurat potrzebowaliśmy
- https://www.youtube.com/@logicprojects - do dodania menu głównego, menu pauzy, menu game over i stanów gry inspirowaliśmy się kilkoma tutorialami z tego kanału, przykładowo:  https://www.youtube.com/watch?v=ZPVI1yd7eFg&t=430s