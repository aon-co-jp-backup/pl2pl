       IDENTIFICATION DIVISION.
       PROGRAM-ID. CALC.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 I PIC 9(3) VALUE 0.
       01 TOTAL PIC 9(5) VALUE 0.
       01 MSG PIC X(10).
       PROCEDURE DIVISION.
       MAIN-PARA.
           PERFORM VARYING I FROM 1 BY 1 UNTIL I > 5
               ADD I TO TOTAL
           END-PERFORM.
           COMPUTE TOTAL = TOTAL * 2 + (I - 1).
           IF TOTAL > 20 AND NOT TOTAL = 99
               MOVE "BIG" TO MSG
           ELSE
               MOVE "SMALL" TO MSG
           END-IF.
           PERFORM SHOW 2 TIMES.
           STOP RUN.
       SHOW.
           DISPLAY "TOTAL=" TOTAL " MSG=" MSG.
