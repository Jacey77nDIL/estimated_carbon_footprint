const userAgent = navigator.userAgent;
const parts = userAgent.split(';');
const beforeLast = parts[parts.length - 2]?.trim(); // Element before the last
const afterLast = parts[parts.length - 1]?.split(')')[0]?.trim(); // Element after the last
const osHeader = document.querySelector(".os");
const nameHeader = document.querySelector(".name");
const laptopNameInput = document.querySelector(".laptop_name");
const batteryCapacityHeader = document.querySelector(".battery-capacity");
const chargeFrequencyHeader = document.querySelector(".charge-frequency");
const dataInput = document.querySelector('.dataNumber');
const wifiCheckbox = document.getElementById('wifi');
const cellularCheckbox = document.getElementById('cellular');
const dataSection = document.querySelector('.data-section');
const finalResult = document.querySelector('.result');

let isLaptop = /Macintosh|Windows/.test(userAgent);
let isIOS = /iPhone|iPad|iPod/.test(userAgent);
let dataEmission;
let noDataEmissionResult;

let userDevice = "";
// Prepare data to send to Rust
const deviceData = {
    screen_width: window.screen.width,
    screen_height: window.screen.height,
    user_device: userDevice,
    is_ios: isIOS,
    is_laptop: isLaptop,
};

if (beforeLast.includes('Android')) {
    userDevice = afterLast;
    deviceData.user_device = userDevice;
    nameHeader.textContent = `Name: ${afterLast}`;
} else if (isLaptop) {
    osHeader.textContent = `OS: ${beforeLast}`;
    laptopNameInput.classList.remove('invisible');
    laptopNameInput.classList.add('display');
    // Ask for user input of their device
    laptopNameInput.addEventListener('input', (event) => {
        const updatedUserDevice = event.target.value.trim(); // Get the user input
    
        if (updatedUserDevice) {
            userDevice = updatedUserDevice; // Update the userDevice with the new value
            deviceData.user_device = userDevice; // Update deviceData with the new userDevice
    
            setTimeout(() => {
                // Send the updated deviceData to the server
                sendToBackend(deviceData);
                console.log(`Updated User Device: ${userDevice}`);
                sendLaptopBatteryCapacity(deviceData);
            }, 10000); // Wait 7 seconds before executing
        }
    });
} else if (isIOS) {
    userDevice = "Iphone";
} else {
    const browserError = 'Browser cannot find your phone';
    console.log(browserError);
}

const mobileCarbonFootprintWithoutDataUsage = (batteryCapacity) => {
    const nominalPhoneVoltage = 3.7;
    const batteryCapacityInWh = batteryCapacity * nominalPhoneVoltage / 1000;
    const averageTeenWhUsage = 18;
    const chargeFrequency = batteryCapacityInWh / averageTeenWhUsage;
    const chargeEfficiency = 0.8; // energy losses of about 20%
    const dailyEnergyConsumption = (batteryCapacityInWh * (1 / chargeFrequency) / chargeEfficiency) / 1000;
    const yearlyEnergyConsumption = dailyEnergyConsumption * 365;
    const carbonIntensityInNigeria = 0.5;
    const finalCarbonEmmision = yearlyEnergyConsumption * carbonIntensityInNigeria;

    return{
        chargeFrequency: chargeFrequency,
        finalCarbonEmmision: finalCarbonEmmision
    };
};

const laptopCarbonFootprintWithoutDataUsage = (batteryCapacity) => {
    const chargeFrequency = batteryCapacity / 50; // 50Wh normal daily usage
    const chargeEfficiency = 0.8; // energy losses of about 20%
    const dailyEnergyConsumption = (batteryCapacity * (1 / chargeFrequency) / chargeEfficiency) / 1000;
    const yearlyEnergyConsumption = dailyEnergyConsumption * 365;
    const carbonIntensityInNigeria = 0.5;
    const finalCarbonEmmision = yearlyEnergyConsumption * carbonIntensityInNigeria;

    return{
        chargeFrequency: chargeFrequency,
        finalCarbonEmmision: finalCarbonEmmision
    };
};

wifiCheckbox.addEventListener('change', function() {
    if (wifiCheckbox.checked) {
        let dataType = 'wifi';
        let dataUsage = dataInput.value;
        dataSection.classList.add('invisible');
        dataEmission = dataUsage * 0.01; // wifi CO2e
        finalResult.textContent = `Result: ${Math.round((noDataEmissionResult + dataEmission) * 100) / 100} kg CO\u2082e/year`;
    }
});

cellularCheckbox.addEventListener('change', function() {
    if (cellularCheckbox.checked) {
        let dataType = 'cellular';
        let dataUsage = dataInput.value;
        dataSection.classList.add('invisible');
        dataEmission = dataUsage * 0.036; // cellular CO2e
        finalResult.textContent = `Result: ${Math.round((noDataEmissionResult + dataEmission) * 100) / 100} kg CO\u2082e/year`; 
    }
});

// Function to send device data to the `/device` endpoint
async function sendToBackend() {
    const response = await fetch('https://estimated-carbon-footprint.onrender.com/device', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(deviceData),
    });

    const result = await response.json(); // Parse the JSON response
    console.log("Received from /device:", result); // Add this line
    console.log(result); // Log the entire result to inspect

    // If we receive valid data, send it to `/ios-options`
    if (result && result.screen_width && result.screen_height) {
        console.log("Triggering sendIosOptions...");
        await sendIosOptions(result);
    }

    // If we receive valid data, send it to `/samsung_devices`
    if (result && result.user_device) {
        console.log("Triggering sendIosOptions...");
        await sendSamsungDevices(result);
    }
}

async function sendSamsungDevices(deviceData) {
    const response = await fetch('https://estimated-carbon-footprint.onrender.com/samsung_devices', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(deviceData),
    });
    const samsung_device_name = await response.json();
    userDevice = samsung_device_name;
    deviceData.user_device = samsung_device_name;
    if (!isIOS && !isLaptop) {
        nameHeader.textContent = `Name: ${samsung_device_name}`;
    }
    matchPhoneToBatteryCapacity(deviceData);
}

// Function to send device data to the `/ios-options` endpoint
async function sendIosOptions(deviceData) {
    const response = await fetch('https://estimated-carbon-footprint.onrender.com/ios-options', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(deviceData),
    });

    const result = await response.json(); // Parse the JSON response
    console.log(result); // Log the entire result to inspect

    // Get the container where we want to add the checkboxes
    const container = document.querySelector('.select-ios');
    container.innerHTML = ''; // Clear any existing content (optional)

    if (isIOS && result) {
        console.log("Found iPhone models: ", result.iphone_model);
        // Display the models in the UI
        const modelsList = document.createElement('ul');
        container.classList.remove('invisible');
        result.iphone_model.forEach(model => {
            // Create checkbox input
            const checkbox = document.createElement('input');
            checkbox.type = 'checkbox';
            checkbox.id = model; // Set the id of the checkbox
            checkbox.name = model; // Set the name of the checkbox
            checkbox.value = model; // Set the value of the checkbox

            // Create label for the checkbox
            const label = document.createElement('label');
            label.setAttribute('for', model); // Set the 'for' attribute of the label
            label.textContent = ` ${model}`; // Set the text of the label

            // Append checkbox and label to the container
            container.appendChild(checkbox);
            container.appendChild(label);
            container.appendChild(document.createElement('br'));

            // Add event listener to checkbox
            checkbox.addEventListener('change', function() {
                if (checkbox.checked) {
                    // If the checkbox is checked, update the name and hide the container
                    nameHeader.textContent = `Name: ${model}`;
                    userDevice = model;
                    deviceData.user_device = model;
                    container.classList.add('invisible'); // Add the 'invisible' class to the container
                    matchPhoneToBatteryCapacity(deviceData);
                }
            });
        });

        // Append the list to a container (make sure you have a div with id="modelsContainer" in your HTML)
        document.getElementById('modelsContainer').appendChild(modelsList);
    } else {
        console.log("No matching iPhone models found");
    }
}

async function matchPhoneToBatteryCapacity(deviceData) {
    const response = await fetch('https://estimated-carbon-footprint.onrender.com/get_device_battery', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(deviceData),
    });
    const battery_capacity = await response.json();
    batteryCapacityHeader.textContent = `Battery Capacity: ${battery_capacity} mAh`;
    const resultWithoutData = mobileCarbonFootprintWithoutDataUsage(battery_capacity);
    chargeFrequencyHeader.textContent = `Charge Frequency: ${Math.round(resultWithoutData.chargeFrequency * 100) / 100}`;
    finalResult.textContent = `Result: ${Math.round(resultWithoutData.finalCarbonEmmision * 100) / 100}`;
    noDataEmissionResult = resultWithoutData.finalCarbonEmmision;
}

async function sendLaptopBatteryCapacity(deviceData) {
    const response = await fetch('https://estimated-carbon-footprint.onrender.com/get_battery_capacity_for_laptops', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(deviceData),
    });

    const battery_capacity_laptops = await response.json();
    batteryCapacityHeader.textContent = `Battery Capacity: ${battery_capacity_laptops} Wh`;
    const resultWithoutData = laptopCarbonFootprintWithoutDataUsage(battery_capacity_laptops);
    chargeFrequencyHeader.textContent = `Charge Frequency: ${Math.round(resultWithoutData.chargeFrequency * 100) / 100}`;
    finalResult.textContent = `Result: ${Math.round(resultWithoutData.finalCarbonEmmision * 100) / 100}`;
    noDataEmissionResult = resultWithoutData.finalCarbonEmmision;
}

// Send data to backend (initial device data)
sendToBackend().then(() => {
    sendIosOptions(deviceData); // Only call if device is iOS
    sendSamsungDevices(deviceData); // Call to handle Samsung devices
});

