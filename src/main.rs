#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::{entry, DefaultHandler_};

extern crate cubeL4;

const RNG : *mut cubeL4::RNG_TypeDef = cubeL4::RNG_BASE as *mut cubeL4::RNG_TypeDef;

#[entry]
fn main() -> ! {
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    // Standard prologue for STM32 HAL
    unsafe {
        cubeL4::HAL_Init();
        SystemClock_Config();
    }

    let xRNG = cubeL4::RNG_HandleTypeDef {
        // Instance: cubeL4::RNG_BASE as *mut cubeL4::RNG_TypeDef,
        Instance : RNG,
        Lock: 0,
        State: 0,
        ErrorCode: 0,
        RandomNumber: 0
    };

    let raw = &xRNG as *const cubeL4::RNG_HandleTypeDef;

    unsafe {
        let _hal_ret = cubeL4::HAL_RNG_Init(raw as *mut cubeL4::RNG_HandleTypeDef );

        let mut rand : u32 = 0;
        let rand_ptr: *mut u32 = &mut rand;

        loop {
            let raw = &xRNG as *const cubeL4::RNG_HandleTypeDef;

            cubeL4::HAL_RNG_GenerateRandomNumber(raw as *mut cubeL4::RNG_HandleTypeDef, rand_ptr );
            asm::nop();
        }
    }

    // loop{};
}

fn SystemClock_Config()
{
    // let mut RCC_OscInitStruct: cubeL4::RCC_OscInitTypeDef;
    // let mut RCC_ClkInitStruct: cubeL4::RCC_ClkInitTypeDef;

    /* Configure the main internal regulator output voltage
     */
    unsafe {
        if cubeL4::HAL_PWREx_ControlVoltageScaling(cubeL4::PWR_REGULATOR_VOLTAGE_SCALE1) != cubeL4::HAL_StatusTypeDef_HAL_OK
        {
            Error_Handler();
        }
    }
    /* Initializes the RCC Oscillators according to the specified parameters
     * in the RCC_OscInitTypeDef structure.
     */
    let mut RCC_OscInitStruct = cubeL4::RCC_OscInitTypeDef {
        OscillatorType : cubeL4::RCC_OSCILLATORTYPE_HSI,
        HSIState : cubeL4::RCC_HSI_ON,
        HSICalibrationValue : cubeL4::RCC_HSICALIBRATION_DEFAULT,
        PLL : cubeL4::RCC_PLLInitTypeDef {
            PLLState : cubeL4::RCC_PLL_ON,
            PLLSource : cubeL4::RCC_PLLSOURCE_HSI,
            PLLM : 1,
            PLLN : 10,
            PLLP : cubeL4::RCC_PLLP_DIV7,
            PLLQ : cubeL4::RCC_PLLQ_DIV4,
            PLLR : cubeL4::RCC_PLLR_DIV2,
        },
        // HSEState: 0,
        // LSEState: 0,
        // LSIState: 0,
        // MSIState: 0,
        // MSICalibrationValue: 0,
        // MSIClockRange: 0,
        // HSI48State: 0,
        ..Default::default() // TODO major, check if this actually gives correct values
    };

    unsafe {
        if cubeL4::HAL_RCC_OscConfig(&mut RCC_OscInitStruct as *mut cubeL4::RCC_OscInitTypeDef) != cubeL4::HAL_StatusTypeDef_HAL_OK
        {
            Error_Handler();
        }
    }

    /* Initializes the CPU, AHB and APB buses clocks
     */
    let mut RCC_ClkInitStruct = cubeL4::RCC_ClkInitTypeDef
    {
        ClockType : cubeL4::RCC_CLOCKTYPE_HCLK | cubeL4::RCC_CLOCKTYPE_SYSCLK | cubeL4::RCC_CLOCKTYPE_PCLK1 | cubeL4::RCC_CLOCKTYPE_PCLK2,
        SYSCLKSource : cubeL4::RCC_SYSCLKSOURCE_PLLCLK,
        AHBCLKDivider : cubeL4::RCC_SYSCLK_DIV1,
        APB1CLKDivider : cubeL4::RCC_HCLK_DIV1,
        APB2CLKDivider : cubeL4::RCC_HCLK_DIV1,
    };

    unsafe {
        if cubeL4::HAL_RCC_ClockConfig(&mut RCC_ClkInitStruct as *mut cubeL4::RCC_ClkInitTypeDef, cubeL4::FLASH_LATENCY_4) != cubeL4::HAL_StatusTypeDef_HAL_OK
        {
            Error_Handler();
        }
    }
}

#[no_mangle]
pub extern fn Error_Handler()
{
    /* USER CODE BEGIN Error_Handler_Debug */
    /* User can add his own implementation to report the HAL error return state */
    // __disable_irq(); // TODO: bindgen does not generate a binding for this ... ?

    asm::bkpt();

    loop {
        asm::nop();
    }
    /* USER CODE END Error_Handler_Debug */
}