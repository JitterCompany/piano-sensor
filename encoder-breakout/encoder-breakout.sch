EESchema Schematic File Version 4
LIBS:encoder-breakout-cache
EELAYER 30 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 1 1
Title ""
Date ""
Rev ""
Comp ""
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
$Comp
L Jitter_Components:AEDR-8300 U1
U 1 1 5D91FC61
P 5350 3650
F 0 "U1" H 5022 3696 50  0000 R CNN
F 1 "AEDR-8300" H 5022 3605 50  0000 R CNN
F 2 "Jitter_Footprints:AEDR-8300" H 5350 3850 50  0001 C CNN
F 3 "https://datasheet.octopart.com/AEDR-8300-1K2-Avago-datasheet-76201850.pdf" H 5350 3850 50  0001 C CNN
	1    5350 3650
	1    0    0    -1  
$EndComp
$Comp
L Device:R_Small R2
U 1 1 5D9203DC
P 5250 2950
F 0 "R2" H 5309 2996 50  0000 L CNN
F 1 "220" H 5309 2905 50  0000 L CNN
F 2 "Resistor_SMD:R_0402_1005Metric" H 5250 2950 50  0001 C CNN
F 3 "~" H 5250 2950 50  0001 C CNN
	1    5250 2950
	1    0    0    -1  
$EndComp
Wire Wire Line
	5250 3050 5250 3100
$Comp
L power:+5V #PWR05
U 1 1 5D921311
P 5250 2750
F 0 "#PWR05" H 5250 2600 50  0001 C CNN
F 1 "+5V" H 5265 2923 50  0000 C CNN
F 2 "" H 5250 2750 50  0001 C CNN
F 3 "" H 5250 2750 50  0001 C CNN
	1    5250 2750
	1    0    0    -1  
$EndComp
Wire Wire Line
	5250 2750 5250 2800
Wire Wire Line
	5250 2800 5450 2800
Wire Wire Line
	5450 2800 5450 3200
Connection ~ 5250 2800
Wire Wire Line
	5250 2800 5250 2850
$Comp
L Device:C_Small C2
U 1 1 5D921D6E
P 5650 2900
F 0 "C2" H 5742 2946 50  0000 L CNN
F 1 "100nF" H 5742 2855 50  0000 L CNN
F 2 "Capacitor_SMD:C_0402_1005Metric" H 5650 2900 50  0001 C CNN
F 3 "~" H 5650 2900 50  0001 C CNN
	1    5650 2900
	1    0    0    -1  
$EndComp
Connection ~ 5450 2800
Wire Wire Line
	5450 2800 5650 2800
$Comp
L power:GND #PWR07
U 1 1 5D922998
P 5650 3000
F 0 "#PWR07" H 5650 2750 50  0001 C CNN
F 1 "GND" H 5655 2827 50  0000 C CNN
F 2 "" H 5650 3000 50  0001 C CNN
F 3 "" H 5650 3000 50  0001 C CNN
	1    5650 3000
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR010
U 1 1 5D922BA5
P 5250 4250
F 0 "#PWR010" H 5250 4000 50  0001 C CNN
F 1 "GND" H 5255 4077 50  0000 C CNN
F 2 "" H 5250 4250 50  0001 C CNN
F 3 "" H 5250 4250 50  0001 C CNN
	1    5250 4250
	1    0    0    -1  
$EndComp
Wire Wire Line
	5250 4100 5250 4150
Wire Wire Line
	5450 4100 5450 4150
Wire Wire Line
	5450 4150 5250 4150
Connection ~ 5250 4150
Wire Wire Line
	5250 4150 5250 4250
$Comp
L Device:LED D1
U 1 1 5D923C30
P 8650 1950
F 0 "D1" V 8689 1833 50  0000 R CNN
F 1 "KPT-1608EC" V 8598 1833 50  0000 R CNN
F 2 "LED_SMD:LED_0603_1608Metric" H 8650 1950 50  0001 C CNN
F 3 "http://www.farnell.com/datasheets/2045911.pdf" H 8650 1950 50  0001 C CNN
F 4 "2099221" V 8650 1950 50  0001 C CNN "Farnell"
	1    8650 1950
	0    -1   -1   0   
$EndComp
$Comp
L Device:R_Small R1
U 1 1 5D9256C2
P 8650 1700
F 0 "R1" H 8709 1746 50  0000 L CNN
F 1 "220" H 8709 1655 50  0000 L CNN
F 2 "Resistor_SMD:R_0402_1005Metric" H 8650 1700 50  0001 C CNN
F 3 "~" H 8650 1700 50  0001 C CNN
	1    8650 1700
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR03
U 1 1 5D925C41
P 8650 2100
F 0 "#PWR03" H 8650 1850 50  0001 C CNN
F 1 "GND" H 8655 1927 50  0000 C CNN
F 2 "" H 8650 2100 50  0001 C CNN
F 3 "" H 8650 2100 50  0001 C CNN
	1    8650 2100
	1    0    0    -1  
$EndComp
Text Notes 8050 1250 0    50   ~ 0
Optional LED to show active channel or blink for interrupts
Text Label 8650 1400 0    50   ~ 0
STATUS_LED_3V3
Wire Wire Line
	8650 1600 8650 1400
Text Label 7000 3550 0    50   ~ 0
Ch_A
Text Label 7000 3750 0    50   ~ 0
Ch_B
$Comp
L Connector_Generic_MountingPin:Conn_01x06_MountingPin J1
U 1 1 5D92BFA7
P 9450 3250
F 0 "J1" H 9478 3226 50  0000 L CNN
F 1 "Conn_01x06_MountingPin" H 9478 3135 50  0000 L CNN
F 2 "Jitter_Footprints:JST_PH_S6B-PH-SM4-TB_1x06-1MP_P2.00mm_Horizontal" H 9450 3250 50  0001 C CNN
F 3 "~" H 9450 3250 50  0001 C CNN
	1    9450 3250
	1    0    0    -1  
$EndComp
Text Label 8900 3250 0    50   ~ 0
Ch_A
Text Label 8900 3450 0    50   ~ 0
Ch_B
$Comp
L power:GND #PWR09
U 1 1 5D92CB6F
P 8400 3400
F 0 "#PWR09" H 8400 3150 50  0001 C CNN
F 1 "GND" H 8405 3227 50  0000 C CNN
F 2 "" H 8400 3400 50  0001 C CNN
F 3 "" H 8400 3400 50  0001 C CNN
	1    8400 3400
	1    0    0    -1  
$EndComp
$Comp
L power:+5V #PWR08
U 1 1 5D92D367
P 8550 3150
F 0 "#PWR08" H 8550 3000 50  0001 C CNN
F 1 "+5V" H 8565 3323 50  0000 C CNN
F 2 "" H 8550 3150 50  0001 C CNN
F 3 "" H 8550 3150 50  0001 C CNN
	1    8550 3150
	1    0    0    -1  
$EndComp
Wire Wire Line
	8900 3450 9250 3450
Wire Wire Line
	9250 3250 8900 3250
Wire Wire Line
	5750 3550 7350 3550
Wire Wire Line
	5750 3750 7350 3750
$Comp
L Device:C_Small C1
U 1 1 5D92EEED
P 7950 2700
F 0 "C1" H 8042 2746 50  0000 L CNN
F 1 "100nF" H 8042 2655 50  0000 L CNN
F 2 "Capacitor_SMD:C_0402_1005Metric" H 7950 2700 50  0001 C CNN
F 3 "~" H 7950 2700 50  0001 C CNN
	1    7950 2700
	1    0    0    -1  
$EndComp
$Comp
L power:+5V #PWR04
U 1 1 5D93015E
P 7950 2500
F 0 "#PWR04" H 7950 2350 50  0001 C CNN
F 1 "+5V" H 7965 2673 50  0000 C CNN
F 2 "" H 7950 2500 50  0001 C CNN
F 3 "" H 7950 2500 50  0001 C CNN
	1    7950 2500
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR06
U 1 1 5D930523
P 7950 2800
F 0 "#PWR06" H 7950 2550 50  0001 C CNN
F 1 "GND" H 7955 2627 50  0000 C CNN
F 2 "" H 7950 2800 50  0001 C CNN
F 3 "" H 7950 2800 50  0001 C CNN
	1    7950 2800
	1    0    0    -1  
$EndComp
$Comp
L Mechanical:MountingHole_Pad H2
U 1 1 5D931CDA
P 5400 1850
F 0 "H2" H 5500 1899 50  0000 L CNN
F 1 "9774060482R" H 5500 1808 50  0000 L CNN
F 2 "Jitter_Footprints:MountingHole_M4_threaded_6mm" H 5400 1850 50  0001 C CNN
F 3 "http://www.farnell.com/datasheets/2580475.pdf" H 5400 1850 50  0001 C CNN
	1    5400 1850
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR01
U 1 1 5D93254D
P 4400 1950
F 0 "#PWR01" H 4400 1700 50  0001 C CNN
F 1 "GND" H 4405 1777 50  0000 C CNN
F 2 "" H 4400 1950 50  0001 C CNN
F 3 "" H 4400 1950 50  0001 C CNN
	1    4400 1950
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR02
U 1 1 5D9329AE
P 5400 1950
F 0 "#PWR02" H 5400 1700 50  0001 C CNN
F 1 "GND" H 5405 1777 50  0000 C CNN
F 2 "" H 5400 1950 50  0001 C CNN
F 3 "" H 5400 1950 50  0001 C CNN
	1    5400 1950
	1    0    0    -1  
$EndComp
$Comp
L Mechanical:MountingHole_Pad H1
U 1 1 5D9317F5
P 4400 1850
F 0 "H1" H 4500 1899 50  0000 L CNN
F 1 "9774060482R" H 4500 1808 50  0000 L CNN
F 2 "Jitter_Footprints:MountingHole_M4_threaded_6mm" H 4400 1850 50  0001 C CNN
F 3 "http://www.farnell.com/datasheets/2580475.pdf" H 4400 1850 50  0001 C CNN
	1    4400 1850
	1    0    0    -1  
$EndComp
Wire Wire Line
	8550 3150 8900 3150
Text Label 8550 3550 0    50   ~ 0
STATUS_LED_3V3
Wire Wire Line
	8550 3550 9250 3550
Wire Wire Line
	8400 3350 8400 3400
$Comp
L power:PWR_FLAG #FLG0101
U 1 1 5D9DAEE1
P 5000 3100
F 0 "#FLG0101" H 5000 3175 50  0001 C CNN
F 1 "PWR_FLAG" H 5000 3273 50  0000 C CNN
F 2 "" H 5000 3100 50  0001 C CNN
F 3 "~" H 5000 3100 50  0001 C CNN
	1    5000 3100
	1    0    0    -1  
$EndComp
Wire Wire Line
	5000 3100 5250 3100
Connection ~ 5250 3100
Wire Wire Line
	5250 3100 5250 3200
NoConn ~ 9250 3050
Wire Wire Line
	7950 2500 7950 2600
$Comp
L power:PWR_FLAG #FLG0102
U 1 1 5D9DD207
P 8900 3150
F 0 "#FLG0102" H 8900 3225 50  0001 C CNN
F 1 "PWR_FLAG" H 8900 3323 50  0000 C CNN
F 2 "" H 8900 3150 50  0001 C CNN
F 3 "~" H 8900 3150 50  0001 C CNN
	1    8900 3150
	1    0    0    -1  
$EndComp
Connection ~ 8900 3150
Wire Wire Line
	8900 3150 9250 3150
$Comp
L power:PWR_FLAG #FLG0103
U 1 1 5D9DD9D3
P 8400 3350
F 0 "#FLG0103" H 8400 3425 50  0001 C CNN
F 1 "PWR_FLAG" H 8400 3523 50  0000 C CNN
F 2 "" H 8400 3350 50  0001 C CNN
F 3 "~" H 8400 3350 50  0001 C CNN
	1    8400 3350
	1    0    0    -1  
$EndComp
Connection ~ 8400 3350
Wire Wire Line
	8400 3350 9250 3350
$Comp
L power:GND #PWR011
U 1 1 5DB9BB58
P 9450 3750
F 0 "#PWR011" H 9450 3500 50  0001 C CNN
F 1 "GND" H 9455 3577 50  0000 C CNN
F 2 "" H 9450 3750 50  0001 C CNN
F 3 "" H 9450 3750 50  0001 C CNN
	1    9450 3750
	1    0    0    -1  
$EndComp
$Comp
L Jitter_Components:LOGO LOGO2
U 1 1 5DB9AA77
P 7500 5000
F 0 "LOGO2" H 7578 5046 50  0000 L CNN
F 1 "DNI" H 7578 4955 50  0000 L CNN
F 2 "Jitter_Logos:JitterLogo" H 7500 5000 50  0001 C CNN
F 3 "" H 7500 5000 50  0001 C CNN
	1    7500 5000
	1    0    0    -1  
$EndComp
$Comp
L Jitter_Components:LOGO LOGO1
U 1 1 5DB9B670
P 7000 5000
F 0 "LOGO1" H 7078 5046 50  0000 L CNN
F 1 "DNI" H 7078 4955 50  0000 L CNN
F 2 "Jitter_Logos:JitterLogo" H 7000 5000 50  0001 C CNN
F 3 "" H 7000 5000 50  0001 C CNN
	1    7000 5000
	1    0    0    -1  
$EndComp
$EndSCHEMATC
