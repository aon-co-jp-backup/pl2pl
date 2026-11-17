       IDENTIFICATION DIVISION.
       PROGRAM-ID. PAYROLL.
       ENVIRONMENT DIVISION.
       INPUT-OUTPUT SECTION.
       FILE-CONTROL.
           SELECT IN-F ASSIGN TO INFILE
               ORGANIZATION IS LINE SEQUENTIAL.
       DATA DIVISION.
       FILE SECTION.
       FD  IN-F.
       01  IN-REC PIC X(20).
       WORKING-STORAGE SECTION.
       01  WS-EOF PIC X VALUE 'N'.
       01  WS-N   PIC 9(4) VALUE 0.
       PROCEDURE DIVISION.
           OPEN INPUT IN-F.
           PERFORM UNTIL WS-EOF = 'Y'
               READ IN-F
                   AT END MOVE 'Y' TO WS-EOF
                   NOT AT END ADD 1 TO WS-N
               END-READ
           END-PERFORM.
           CLOSE IN-F.
           DISPLAY 'RECORDS=' WS-N.
           STOP RUN.
