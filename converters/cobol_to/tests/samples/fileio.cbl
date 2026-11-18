       IDENTIFICATION DIVISION.
       PROGRAM-ID. FIO.
       ENVIRONMENT DIVISION.
       INPUT-OUTPUT SECTION.
       FILE-CONTROL.
           SELECT OUT-FILE ASSIGN TO "fio_test.dat"
               ORGANIZATION IS LINE SEQUENTIAL
               FILE STATUS IS FS.
           SELECT IN-FILE ASSIGN TO "fio_test.dat"
               ORGANIZATION IS LINE SEQUENTIAL.
       DATA DIVISION.
       FILE SECTION.
       FD OUT-FILE.
       01 OUT-REC.
          05 O-NAME PIC X(8).
          05 O-AMT PIC 9(4)V99.
       FD IN-FILE.
       01 IN-REC.
          05 I-NAME PIC X(8).
          05 I-AMT PIC 9(4)V99.
       WORKING-STORAGE SECTION.
       01 FS PIC XX.
       01 EOF-FLAG PIC 9 VALUE 0.
          88 AT-EOF VALUE 1.
       PROCEDURE DIVISION.
       MAIN.
           OPEN OUTPUT OUT-FILE.
           MOVE "APPLE" TO O-NAME.
           MOVE 3.5 TO O-AMT.
           WRITE OUT-REC.
           MOVE "PEAR" TO O-NAME.
           MOVE 12.25 TO O-AMT.
           WRITE OUT-REC.
           CLOSE OUT-FILE.
           DISPLAY "FS=" FS.
           OPEN INPUT IN-FILE.
           PERFORM UNTIL AT-EOF
               READ IN-FILE
                   AT END SET AT-EOF TO TRUE
                   NOT AT END DISPLAY I-NAME " " I-AMT
               END-READ
           END-PERFORM.
           CLOSE IN-FILE.
           STOP RUN.
