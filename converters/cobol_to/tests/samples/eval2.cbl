       IDENTIFICATION DIVISION.
       PROGRAM-ID. EVAL2.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 N PIC 9(3) VALUE 7.
       01 R PIC X(10).
       PROCEDURE DIVISION.
           EVALUATE N
               WHEN 1 WHEN 2
                   MOVE "LOW" TO R
               WHEN 5 THRU 9
                   MOVE "MID" TO R
               WHEN OTHER
                   MOVE "HIGH" TO R
           END-EVALUATE.
           DISPLAY "R=" R.
           EVALUATE TRUE
               WHEN N > 100 DISPLAY "BIG"
               WHEN N < 10 DISPLAY "SMALL"
           END-EVALUATE.
           STOP RUN.
