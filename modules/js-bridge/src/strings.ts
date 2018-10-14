export const weather = {
    0: 'Clear',
    1: 'Light cloud',
    2: 'Overcast',
    3: 'Light rain',
    4: 'Heavy rain',
    5: 'Storm',
};

export const sessionType = {
    0: 'Unknown',
    1: 'P1',
    2: 'P2',
    3: 'P3',
    4: 'Short practise',
    5: 'Q1',
    6: 'Q2',
    7: 'Q3',
    8: 'Short qualification',
    9: 'One shot qualification',
    10: 'Race',
    11: 'Race2',
    12: 'Time trial',
};

export const tractionControl = {
    0: 'Low',
    1: 'Medium',
    2: 'High',
};

export const antiLockBreaks = {
    0: 'Off',
    1: 'On',
};

export const fuelMix = {
    0: 'Lean',
    1: 'Standard',
    2: 'Rich',
    3: 'Max',
};

export const pitLimitterStatus = {
    0: 'Off',
    1: 'On',
};

export const tyreCompound = {
    0: 'Hypersoft',
    1: 'Ultrasoft',
    2: 'SuperSoft',
    3: 'Soft',
    4: 'Medium',
    5: 'Hard',
    6: 'SuperHard',
    7: 'Inter',
    8: 'Wet',
};

export const flag = {
    '-1': 'Unknown',
    0: 'None',
    1: 'Green',
    2: 'Blue',
    3: 'Yellow',
    4: 'Red',
};

export const ersMode = {
    0: 'None',
    1: 'Low',
    2: 'Medium',
    3: 'High',
    4: 'Overtake',
    5: 'Hotlap',
};

export const era = {
    0: 'Modern',
    1: 'Classic',
};

export const safetyCarStatus = {
    0: 'No safety car',
    1: 'Full safety car',
    3: 'VSC',
};

export const driverStatus = {
    0: 'In garage',
    1: 'Flying lap',
    2: 'In lap',
    3: 'Outlap',
    4: 'On track',
};

export const resultStatus = {
    0: 'Invalid',
    1: 'Inactive',
    2: 'Active',
    3: 'Finished',
    4: 'Disqualified',
    5: 'Not Clasified',
    6: 'Retired',
};

export const track = {
    '-1': 'Unknown',
    0: 'Australia',
    1: 'France',
    2: 'China',
    3: 'Bahrain',
    4: 'Spain',
    5: 'Monaco',
    6: 'Canada',
    7: 'GB',
    8: 'Germany',
    9: 'Hungary',
    10: 'Belgium',
    11: 'Italy',
    12: 'Singapore',
    13: 'Japan',
    14: 'UAE',
    15: 'USA',
    16: 'Brazil',
    17: 'Austria',
    18: 'Russia',
    19: 'Mexico',
    20: 'Azerbaijan',
    21: 'Bahrain short',
    22: 'GB short',
    23: 'USA short',
    24: 'Japan short',
};

export const team = {
    0: 'Mercedes',
    1: 'Ferrari',
    2: 'Red Bull',
    3: 'Williams',
    4: 'Force India',
    5: 'Renault',
    6: 'Toro Rosso',
    7: 'Haas',
    8: 'McLaren',
    9: 'Sauber',
    10: 'McLaren 1988',
    11: 'McLaren 1991',
    12: 'Williams 1992',
    13: 'Ferrari 1995',
    14: 'Williams 1996',
    15: 'McLaren 1998',
    16: 'Ferrari 2002',
    17: 'Ferrari 2004',
    18: 'Renault 2006',
    19: 'Ferrari 2007',
    20: 'McLaren 2008',
    21: 'Red Bull 2010',
    22: 'Ferrari 1976',
    34: 'McLaren 1976',
    35: 'Lotus 1972',
    36: 'Ferrari 1979',
    37: 'McLaren 1982',
    38: 'Williams 2003',
    39: 'Brawn 2009',
    40: 'Lotus 1978',
};

export const idStringMap = {
    track_id: track,
    session_type: sessionType,
    era: era,
    weather: weather,
    safety_car_status: safetyCarStatus,
    driver_status: driverStatus,
    result_status: resultStatus,
    pit_limiter_status: pitLimitterStatus,
    tyre_compound: tyreCompound,
    flags: flag,
    ers_mode: ersMode,
    team_id: team,
    traction_control: tractionControl,
    antilock_brakes: antiLockBreaks,
    fuel_mix: fuelMix,
} as { [key: string]: { [key: number]: 'string' } };
