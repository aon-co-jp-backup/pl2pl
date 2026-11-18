       IDENTIFICATION DIVISION.
       PROGRAM-ID. FEAT.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       COPY ITEMS.
       01 TOTAL PIC 9(5)V99 VALUE 0.
       01 AVG PIC 9(3)V9 VALUE 0.
       01 Q PIC 9(3) VALUE 0.
       01 R PIC 9(3) VALUE 0.
       01 TAB OCCURS 3 TIMES PIC 9(3) VALUE 0.
       01 IDX PIC 9 VALUE 0.
       01 FLAG PIC 9 VALUE 0.
          88 IS-ON VALUE 1.
       01 MSG PIC X(30).
       01 NAME1 PIC X(10).
       01 NAME2 PIC X(10).
       01 CNT PIC 9(2) VALUE 0.
       01 WORD PIC X(10) VALUE "banana".
       01 CODE-IN PIC X(4) VALUE "12a4".
       PROCEDURE DIVISION.
       MAIN-PARA.
           COMPUTE TOTAL = PRICE * QTY.
           DISPLAY "TOTAL=" TOTAL.
           COMPUTE AVG ROUNDED = TOTAL / 7.
           DISPLAY "AVG=" AVG.
           DIVIDE 17 BY 5 GIVING Q REMAINDER R.
           DISPLAY "Q=" Q " R=" R.
           PERFORM VARYING IDX FROM 1 BY 1 UNTIL IDX > 3
               COMPUTE TAB(IDX) = IDX * 10
           END-PERFORM.
           DISPLAY "TAB2=" TAB(2).
           SET IS-ON TO TRUE.
           IF IS-ON DISPLAY "ON" ELSE DISPLAY "OFF".
           STRING "Hello" DELIMITED BY SIZE
                  ", " DELIMITED BY SIZE
                  "World" DELIMITED BY SIZE
                  INTO MSG.
           DISPLAY MSG.
           UNSTRING "ab,cd" DELIMITED BY "," INTO NAME1 NAME2.
           DISPLAY NAME1 "/" NAME2.
           INSPECT WORD TALLYING CNT FOR ALL "a".
           DISPLAY "A-COUNT=" CNT.
           INSPECT WORD REPLACING ALL "a" BY "o".
           DISPLAY WORD.
           DISPLAY FUNCTION UPPER-CASE(WORD) " " FUNCTION LENGTH(WORD).
           IF CODE-IN IS NUMERIC DISPLAY "NUM" ELSE DISPLAY "NOTNUM".
           INITIALIZE TOTAL.
           DISPLAY "T0=" TOTAL.
           PERFORM STEP-A THRU STEP-B.
           GO TO FINISH.
           DISPLAY "SKIPPED".
       STEP-A.
           DISPLAY "A".
       STEP-B.
           DISPLAY "B".
       FINISH.
           DISPLAY "DONE".
           STOP RUN.
