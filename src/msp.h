//
// Created by Andrew Pullin on 9/12/21.
//

#include "stm32l4xx_hal.h"

void HAL_MspInit(void);
void HAL_RNG_MspInit(RNG_HandleTypeDef* hrng);

extern void Error_Handler(void);

//TODO rehome?
//void SystemClock_Config(void);