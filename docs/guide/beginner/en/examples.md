# All examples step by step

Each of the 286 examples of the library, with the explanation shown on the **Overview** tab of the application. The examples are grouped by lesson and level.

## Weighing a powder (Beginner)

A very simple budget: the mass of a powder is the reading of the filled container minus the reading of the empty one, with a small correction for the drift of the balance. The two readings have the same uncertainty; together they give √2 times one of them, not twice. GUM and Monte Carlo give the same result, because the model is linear.

<a id="nist-tn1900-e1-weighing"></a>

### Mass of a powder by weighing (NIST TN 1900 E1)

`nist-tn1900-e1-weighing`

#### What is this project about?

TN 1900 E1: mass of a powder from three weighings with a single-pan balance; linear model, GUM and MCM agree.


#### Step 1. What do we measure, and how is it calculated?

The result is **m_P** in g (Mass of the powder). It is not measured directly: it is calculated from the input quantities with the measurement model

```
m_P = c_P - c_E - dc_R
```


#### Step 2. The input quantities and their uncertainties

The model has 3 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **c_P**: Reading with the powder in the container. Best value 53.768 g with standard uncertainty u = 0.005015 g (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **c_E**: Reading of the empty container. Best value 3.436 g with standard uncertainty u = 0.005015 g (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **dc_R**: Drift c_R2 − c_R1 of the reference-weight readings. Best value 0.048 g with standard uncertainty u = 0.0005477 g (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 50.284 g
- standard uncertainty u = 0.0071 g

Reference: NIST TN 1900 E1: m_P = 50.284 g, u = 0.0071 g, 95 % interval [50.270, 50.298] g.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Adding inputs: normal and rectangular (Beginner)

Four inputs are simply added. The three variants differ only in the shape of the input distributions. Compare the GUM result with Monte Carlo: when one rectangular input dominates, the result is not normal any more and the 95 % interval is narrower than 2u. This is the classic case where k = 2 is not right.

<a id="jcgm101-9.2.2-additive-gaussian"></a>

### Additive model with Gaussian inputs

`jcgm101-9.2.2-additive-gaussian`

#### What is this project about?

JCGM 101 9.2.2: four independent inputs, each N(0, 1).


#### Step 1. What do we measure, and how is it calculated?

The result is **Y**. It is not measured directly: it is calculated from the input quantities with the measurement model

```
Y = X1 + X2 + X3 + X4
```


#### Step 2. The input quantities and their uncertainties

The model has 4 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **X1**. Best value 0 with standard uncertainty u = 1 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **X2**. Best value 0 with standard uncertainty u = 1 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **X3**. Best value 0 with standard uncertainty u = 1 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **X4**. Best value 0 with standard uncertainty u = 1 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 0
- standard uncertainty u = 2

Reference: JCGM 101:2008 Table 2 (exact: N(0, 4)); MCM tolerances ≈ 5 × Monte Carlo dispersion at M = 10⁶.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="jcgm101-9.2.3-additive-rectangular"></a>

### Additive model with rectangular inputs

`jcgm101-9.2.3-additive-rectangular`

#### What is this project about?

JCGM 101 9.2.3: four independent inputs, each rectangular with standard deviation 1.


#### Step 1. What do we measure, and how is it calculated?

The result is **Y**. It is not measured directly: it is calculated from the input quantities with the measurement model

```
Y = X1 + X2 + X3 + X4
```


#### Step 2. The input quantities and their uncertainties

The model has 4 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **X1**. Only limits are known: the value lies between -1.7321 and 1.7321 (0 ± 1.7321), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 1.7321/√3 = 1.00.
2. **X2**. Only limits are known: the value lies between -1.7321 and 1.7321 (0 ± 1.7321), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 1.7321/√3 = 1.00.
3. **X3**. Only limits are known: the value lies between -1.7321 and 1.7321 (0 ± 1.7321), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 1.7321/√3 = 1.00.
4. **X4**. Only limits are known: the value lies between -1.7321 and 1.7321 (0 ± 1.7321), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 1.7321/√3 = 1.00.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.

It also uses the **Monte Carlo method**: it draws 2 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 2 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 0
- standard uncertainty u = 2

Reference: JCGM 101:2008 Table 3 and Annex E (analytic endpoints ±3.8812).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="jcgm101-9.2.4-additive-dominant-rectangular"></a>

### Additive model with a dominant rectangular input

`jcgm101-9.2.4-additive-dominant-rectangular`

#### What is this project about?

JCGM 101 9.2.4: as 9.2.3 but X4 has standard deviation 10.


#### Step 1. What do we measure, and how is it calculated?

The result is **Y**. It is not measured directly: it is calculated from the input quantities with the measurement model

```
Y = X1 + X2 + X3 + X4
```


#### Step 2. The input quantities and their uncertainties

The model has 4 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **X1**. Only limits are known: the value lies between -1.7321 and 1.7321 (0 ± 1.7321), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 1.7321/√3 = 1.00.
2. **X2**. Only limits are known: the value lies between -1.7321 and 1.7321 (0 ± 1.7321), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 1.7321/√3 = 1.00.
3. **X3**. Only limits are known: the value lies between -1.7321 and 1.7321 (0 ± 1.7321), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 1.7321/√3 = 1.00.
4. **X4**. Only limits are known: the value lies between -17.321 and 17.321 (0 ± 17.321), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 17.321/√3 = 10.0.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.

It also uses the **Monte Carlo method**: it draws 2 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 2 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- standard uncertainty u = 10.149

Reference: JCGM 101:2008 Table 4 (u(y) = √103 = 10.149).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Calibrating a weight (Beginner)

A 10 kg weight is compared with a reference weight. It shows the typical calibration budget: the certificate of the standard (U and k), drift since its last calibration, the comparator, air buoyancy and the repeated comparisons. The certificate of the standard is the largest contribution.

<a id="ea402-s2-weight-10kg"></a>

### Calibration of a 10 kg weight

`ea402-s2-weight-10kg`

#### What is this project about?

EA-4/02 S2: OIML class M1 weight calibrated against an F2 standard with a mass comparator (ABBA).


#### Step 1. What do we measure, and how is it calculated?

The result is **m_X** in mg (Deviation of the weight from 10 kg). It is not measured directly: it is calculated from the input quantities with the measurement model

```
m_X = m_S + dm_D + dm + dm_C + dB
```


#### Step 2. The input quantities and their uncertainties

The model has 5 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **m_S**: Standard (certificate: +5 mg, U = 45 mg, k = 2). Best value 5 mg. The source (e.g. a certificate) gives the expanded uncertainty U = 45 mg with k = 2, so the standard uncertainty is u = U/k = 45/2 = 22.5 mg (normal distribution).
2. **dm_D**: Drift of the standard since its last calibration. Only limits are known: the value lies between -15 and 15 mg (0 ± 15), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 15/√3 = 8.66 mg.
3. **dm**: Observed difference (mean of 3, s_p = 25 mg). Best value 20 mg with standard uncertainty u = 14.434 mg (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **dm_C**: Comparator eccentricity and magnetic effects. Only limits are known: the value lies between -10 and 10 mg (0 ± 10), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 10/√3 = 5.77 mg.
5. **dB**: Air buoyancy. Only limits are known: the value lies between -10 and 10 mg (0 ± 10), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 10/√3 = 5.77 mg.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 25 mg
- standard uncertainty u = 29.2 mg
- expanded uncertainty U = 58 mg

Reference: EA-4/02 M:2022 S2.10–S2.11 (u = 29.2 mg, U = 58 mg).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## A 10 kg weight in UKAS M3003 (Beginner)

The same kind of weight calibration as in EA-4/02, presented in the British guide. Comparing the two shows that different laboratories make slightly different choices of components and distributions, and that the result is still similar.

<a id="m3003-k4-weight-10kg"></a>

### 10 kg weight of OIML class M1 (UKAS M3003 K.4)

`m3003-k4-weight-10kg`

#### What is this project about?

Comparison with a class F2 standard on a mass comparator, three bracketed comparisons.


#### Step 1. What do we measure, and how is it calculated?

The result is **W_X** in g (Mass of the unknown weight). It is not measured directly: it is calculated from the input quantities with the measurement model

```
W_X = W_S + dW + dD_S + dI_d + dC + dA_b
```


#### Step 2. The input quantities and their uncertainties

The model has 6 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **W_S**: Standard weight (certificate, U = 30 mg, k = 2). Best value 10000.005 g. The source (e.g. a certificate) gives the expanded uncertainty U = 0.03 g with k = 2, so the standard uncertainty is u = U/k = 0.03/2 = 0.0150 g (normal distribution).
2. **dW**: Mean difference of three comparisons (repeatability s = 8.7 mg, ν = 9). Mean of 3 readings, 0.02 g. Instead of the scatter of these few readings, a standard deviation s_p = 0.0087 g determined earlier from more data (ν = 9) is used, so u = s_p/√n = 0.00502 g.
3. **dD_S**: Drift of the standard. Only limits are known: the value lies between -0.03 and 0.03 g (0 ± 0.03), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.03/√3 = 0.0173 g.
4. **dI_d**: Rounding of the two indications (10 mg digit). The value lies between -0.01 and 0.01 g, and values near the centre 0 are more likely than values near the limits (triangular distribution). u = a/√6 = 0.01/√6 = 0.00408 g.
5. **dC**: Comparator non-linearity. Only limits are known: the value lies between -0.003 and 0.003 g (0 ± 0.003), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.003/√3 = 0.00173 g.
6. **dA_b**: Air buoyancy (±1 ppm of nominal value). Only limits are known: the value lies between -0.01 and 0.01 g (0 ± 0.01), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.01/√3 = 0.00577 g.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 200 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 200 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 10000.025 g
- standard uncertainty u = 0.02456 g
- expanded uncertainty U = 0.04912 g

Reference: UKAS M3003 ed. 6 K.4.10–K.4.11: u_c = 24.56 mg, U = 49.12 mg; 10 000.025 g ± 0.049 g.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Calibrating a vernier calliper (Beginner)

A gauge block is measured with a calliper. Most components are rectangular: the resolution of the scale, the temperature difference, the flatness of the jaws. A good example of how a reading on a scale is turned into an uncertainty.

<a id="ea402-s10-vernier-calliper"></a>

### Calibration of a vernier calliper at 150 mm

`ea402-s10-vernier-calliper`

#### What is this project about?

EA-4/02 S10: error of a vernier calliper measured with a gauge block; two rectangular terms dominate (trapezoidal output).


#### Step 1. What do we measure, and how is it calculated?

The result is **E_X** in µm (Error of indication). It is not measured directly: it is calculated from the input quantities with the measurement model

```
E_X = l_iX - l_S + 150000*11.5e-6*dt + dl_iX + dl_M
```


#### Step 2. The input quantities and their uncertainties

The model has 5 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **l_iX**: Indication. An exact value, 150100 µm, with no uncertainty (for example a defined constant or a nominal value).
2. **l_S**: Length of the gauge block. Only limits are known: the value lies between 149999.2 and 150000.8 µm (150000 ± 0.8), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.8/√3 = 0.462 µm.
3. **dt**: Temperature difference calliper–block. Only limits are known: the value lies between -2 and 2 K (0 ± 2), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 2/√3 = 1.15 K.
4. **dl_iX**: Resolution of the scale. Only limits are known: the value lies between -25 and 25 µm (0 ± 25), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 25/√3 = 14.4 µm.
5. **dl_M**: Mechanical effects (force, flatness, parallelism). Only limits are known: the value lies between -50 and 50 µm (0 ± 50), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 50/√3 = 28.9 µm.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 100 µm
- standard uncertainty u = 32 µm

Reference: EA-4/02 M:2022 S10.10–S10.12 (u = 32 µm).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Calibrating a standard resistor (Beginner)

A 10 kΩ resistor is compared with a reference resistor. The model is a product, so the sensitivity coefficients are not 1; look at how the budget converts each relative effect into ohms.

<a id="ea402-s3-resistor-10kohm"></a>

### Calibration of a 10 kΩ standard resistor

`ea402-s3-resistor-10kohm`

#### What is this project about?

EA-4/02 S3: four-terminal resistor calibrated by substitution against a standard, using a DMM as a ratio device.


#### Step 1. What do we measure, and how is it calculated?

The result is **R_X** in mΩ (Resistance of the unknown (mΩ, offset 10 kΩ = 10⁷ mΩ)). It is not measured directly: it is calculated from the input quantities with the measurement model

```
R_X = (R_S + dR_D + dR_TS) * r_C * r - dR_TX
```


#### Step 2. The input quantities and their uncertainties

The model has 6 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **R_S**: Standard resistor (certificate). Best value 10000053 mΩ. The source (e.g. a certificate) gives the expanded uncertainty U = 5 mΩ with k = 2, so the standard uncertainty is u = U/k = 5/2 = 2.50 mΩ (normal distribution).
2. **dR_D**: Drift since last calibration. Only limits are known: the value lies between 10 and 30 mΩ (20 ± 10), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 10/√3 = 5.77 mΩ.
3. **dR_TS**: Temperature effect on the standard. Only limits are known: the value lies between -2.75 and 2.75 mΩ (0 ± 2.75), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 2.75/√3 = 1.59 mΩ.
4. **r_C**: Correction of the ratio (DMM). The value lies between 0.999999 and 1.000001, and values near the centre 1 are more likely than values near the limits (triangular distribution). u = a/√6 = 0.000001/√6 = 4.08 × 10⁻⁷.
5. **r**: Observed ratio R_iX/R_iS (5 readings). Measured 5 times (a **Type A** evaluation, from statistics). The estimate is the mean 1.00001; the readings scatter with a standard deviation s = 1.58 × 10⁻⁷, and the standard uncertainty of the mean is u = s/√n = 1.58 × 10⁻⁷/√5 = 7.07 × 10⁻⁸, with ν = n − 1 = 4 degrees of freedom.
6. **dR_TX**: Temperature effect on the unknown. Only limits are known: the value lies between -5.5 and 5.5 mΩ (0 ± 5.5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 5.5/√3 = 3.18 mΩ.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 10000178 mΩ
- standard uncertainty u = 8.33 mΩ
- expanded uncertainty U = 17 mΩ

Reference: EA-4/02 M:2022 S3.10–S3.11 (u = 8.33 mΩ, U = 0.017 Ω).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Calibrating a digital multimeter (Beginner)

A hand-held multimeter is checked at 100 V against a calibrator. The resolution of the display is often the largest term: a value shown with one decimal can be anywhere within half a digit.

<a id="ea402-s9-dmm-100v"></a>

### Calibration of a hand-held digital multimeter at 100 V

`ea402-s9-dmm-100v`

#### What is this project about?

EA-4/02 S9: error of indication of a 3½-digit DMM; the resolution term dominates and the output is nearly rectangular.


#### Step 1. What do we measure, and how is it calculated?

The result is **E_X** in V (Error of indication). It is not measured directly: it is calculated from the input quantities with the measurement model

```
E_X = V_iX - V_S + dV_iX - dV_S
```


#### Step 2. The input quantities and their uncertainties

The model has 4 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **V_iX**: Indication of the DMM. An exact value, 100.1 V, with no uncertainty (for example a defined constant or a nominal value).
2. **V_S**: Voltage of the calibrator. Best value 100 V. The source (e.g. a certificate) gives the expanded uncertainty U = 0.002 V with k = 2, so the standard uncertainty is u = U/k = 0.002/2 = 0.00100 V (normal distribution).
3. **dV_iX**: Resolution of the DMM. Only limits are known: the value lies between -0.05 and 0.05 V (0 ± 0.05), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.05/√3 = 0.0289 V.
4. **dV_S**: Calibrator: specification and drift. Only limits are known: the value lies between -0.011 and 0.011 V (0 ± 0.011), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.011/√3 = 0.00635 V.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 0.1 V
- standard uncertainty u = 0.03 V

Reference: EA-4/02 M:2022 S9.12–S9.14 (u = 0.030 V).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## A multimeter at 10 V (TÜBİTAK UME) (Beginner)

An example from the uncertainty procedure of TÜBİTAK UME: calibration of a digital multimeter at 10 V DC. It follows the same steps as the guides and is a good template for an electrical calibration certificate.

<a id="ume-procedure-19-1-dmm-10v-calibration"></a>

### 10 V DC calibration of a digital multimeter (TÜBİTAK UME uncertainty procedure, Example 19.1)

`ume-procedure-19-1-dmm-10v-calibration`

#### What is this project about?

Additive model with absolute uncertainties: the error of indication E = V_DMM − V_ref with corrections for the calibrator certificate, its drift and the resolution.


#### Step 1. What do we measure, and how is it calculated?

The result is **E** in µV (Error of indication of the multimeter at 10 V DC, (23 ± 1) °C). It is not measured directly: it is calculated from the input quantities with the measurement model

```
E = x_DMM + d_res - d_cal - d_drift
```


#### Step 2. The input quantities and their uncertainties

The model has 4 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **x_DMM**: Mean of 12 readings minus 10 V (10.000 212 V; s = 27.9 µV). Best value 212 µV with standard uncertainty u = 8.05 µV. Because it is based on few data (ν = 11 degrees of freedom), a Student t distribution is used: it has heavier tails than the normal distribution and makes the coverage factor larger.
2. **d_cal**: Calibrator certificate, U = 5 µV/V at 10 V (k = 2). Best value 0 µV. The source (e.g. a certificate) gives the expanded uncertainty U = 50 µV with k = 2, so the standard uncertainty is u = U/k = 50/2 = 25.0 µV (normal distribution).
3. **d_drift**: Annual drift of the calibrator, ±40 µV. Only limits are known: the value lies between -40 and 40 µV (0 ± 40), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 40/√3 = 23.1 µV.
4. **d_res**: Resolution of the multimeter, 10 µV. Only limits are known: the value lies between -5 and 5 µV (0 ± 5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 5/√3 = 2.89 µV.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 212 µV
- standard uncertainty u = 35.1 µV
- expanded uncertainty U = 70.2 µV

Reference: UME procedure, Section 19.1: u_c = 35.1 µV, ν_eff ≈ 4000, U = 70.2 µV (k = 2); E = 0.21 mV.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Calibrating a gauge block (Beginner)

A 50 mm gauge block is compared with a reference block. Temperature matters: a small difference in temperature or in expansion coefficient changes the length, which is why the budget has thermal terms.

<a id="ea402-s4-gauge-block-50mm"></a>

### Calibration of a 50 mm gauge block

`ea402-s4-gauge-block-50mm`

#### What is this project about?

EA-4/02 S4: grade 0 steel gauge block compared with a calibrated standard; a product of two zero-mean quantities needs second-order terms.


#### Step 1. What do we measure, and how is it calculated?

The result is **l_X** in nm (Length of the gauge block). It is not measured directly: it is calculated from the input quantities with the measurement model

```
l_X = l_S + dl_D + dl + dl_C - 50000000*(alpha*dt + d_alpha*theta) - dl_V
```


#### Step 2. The input quantities and their uncertainties

The model has 9 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **l_S**: Standard (certificate: 50.000 02 mm, U = 30 nm). Best value 50000020 nm with standard uncertainty u = 15 nm (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **dl_D**: Drift of the standard. The value lies between -30 and 30 nm, and values near the centre 0 are more likely than values near the limits (triangular distribution). u = a/√6 = 30/√6 = 12.2 nm.
3. **dl**: Observed difference (mean of 5, s_p = 12 nm). Best value -94 nm with standard uncertainty u = 5.3666 nm (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **dl_C**: Comparator linearity and offset. Only limits are known: the value lies between -32 and 32 nm (0 ± 32), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 32/√3 = 18.5 nm.
5. **alpha**: Mean expansion coefficient. An exact value, 0.0000115 1/K, with no uncertainty (for example a defined constant or a nominal value).
6. **dt**: Temperature difference of the blocks. Only limits are known: the value lies between -0.05 and 0.05 K (0 ± 0.05), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.05/√3 = 0.0289 K.
7. **d_alpha**: Difference of expansion coefficients. The value lies between -0.000002 and 0.000002 1/K, and values near the centre 0 are more likely than values near the limits (triangular distribution). u = a/√6 = 0.000002/√6 = 8.16 × 10⁻⁷ 1/K.
8. **theta**: Deviation of the mean temperature from 20 °C. Only limits are known: the value lies between -0.5 and 0.5 K (0 ± 0.5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.5/√3 = 0.289 K.
9. **dl_V**: Non-central contact. Only limits are known: the value lies between -6.7 and 6.7 nm (0 ± 6.7), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 6.7/√3 = 3.87 nm.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).
6. Because the model is clearly nonlinear, second-order terms are added (GUM 5.1.2 and its 2026 amendment).

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 49999926 nm
- standard uncertainty u = 34.3 nm
- expanded uncertainty U = 69 nm

Reference: EA-4/02 M:2022 S4.12–S4.14 (u = 34.3 nm, U = 69 nm).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## A temperature block calibrator (Beginner)

The temperature of a dry block at 180 °C is measured with a reference thermometer. The budget lists the effects of the block itself: axial and radial non-uniformity, stability, loading.

<a id="ea402-s11-block-calibrator-180c"></a>

### Calibration of a temperature block calibrator at 180 °C

`ea402-s11-block-calibrator-180c`

#### What is this project about?

EA-4/02 S11: temperature of the block measured with a calibrated platinum resistance thermometer (values in mK).


#### Step 1. What do we measure, and how is it calculated?

The result is **t_X** in mK (Temperature of the block (in mK)). It is not measured directly: it is calculated from the input quantities with the measurement model

```
t_X = t_S + dt_S + dt_D - dt_iX + dt_R + dt_A + dt_H + dt_V
```


#### Step 2. The input quantities and their uncertainties

The model has 8 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **t_S**: Reference thermometer reading. Best value 180100 mK. The source (e.g. a certificate) gives the expanded uncertainty U = 30 mK with k = 2, so the standard uncertainty is u = U/k = 30/2 = 15.0 mK (normal distribution).
2. **dt_S**: Calibration of the reference thermometer. Best value 0 mK with standard uncertainty u = 10 mK (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **dt_D**: Drift of the reference thermometer. Only limits are known: the value lies between -40 and 40 mK (0 ± 40), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 40/√3 = 23.1 mK.
4. **dt_iX**: Resolution of the calibrator display. Only limits are known: the value lies between -50 and 50 mK (0 ± 50), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 50/√3 = 28.9 mK.
5. **dt_R**: Radial temperature differences. Only limits are known: the value lies between -100 and 100 mK (0 ± 100), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 100/√3 = 57.7 mK.
6. **dt_A**: Axial inhomogeneity. Only limits are known: the value lies between -250 and 250 mK (0 ± 250), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 250/√3 = 144 mK.
7. **dt_H**: Hysteresis. Only limits are known: the value lies between -50 and 50 mK (0 ± 50), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 50/√3 = 28.9 mK.
8. **dt_V**: Temperature variation during measurement. Only limits are known: the value lies between -30 and 30 mK (0 ± 30), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 30/√3 = 17.3 mK.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 180100 mK
- standard uncertainty u = 164 mK

Reference: EA-4/02 M:2022 S11.9–S11.11 (u = 164 mK).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Calibrating a water meter (Beginner)

Three related projects: the reference volume, the relative error of one run and the mean error of three runs. They show how a long calculation can be split into steps, each with its own budget.

<a id="ea402-s12-water-meter-error"></a>

### Water meter calibration: relative error of one run

`ea402-s12-water-meter-error`

#### What is this project about?

EA-4/02 S12 (second budget): relative error of indication of the meter for a single run.


#### Step 1. What do we measure, and how is it calculated?

The result is **e_X** (Relative error of indication). It is not measured directly: it is calculated from the input quantities with the measurement model

```
e_X = (V_iX + dV_iX2 - dV_iX1)/V_X - 1
```


#### Step 2. The input quantities and their uncertainties

The model has 4 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **V_iX**: Indicated volume. An exact value, 200 L, with no uncertainty (for example a defined constant or a nominal value).
2. **dV_iX1**: Resolution at start. Only limits are known: the value lies between -0.1 and 0.1 L (0 ± 0.1), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.1/√3 = 0.0577 L.
3. **dV_iX2**: Resolution at stop. Only limits are known: the value lies between -0.1 and 0.1 L (0 ± 0.1), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.1/√3 = 0.0577 L.
4. **V_X**: Volume at the meter (first budget). Best value 199.93 L with standard uncertainty u = 0.109 L (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 0.0003
- standard uncertainty u = 0.00068

Reference: EA-4/02 M:2022 S12.10 (e_X = 0.0003, u = 0.68 × 10⁻³).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="ea402-s12-water-meter-mean-error"></a>

### Water meter calibration: mean of three runs

`ea402-s12-water-meter-mean-error`

#### What is this project about?

EA-4/02 S12 (third budget): mean relative error of three runs, with the Welch–Satterthwaite formula and k from the t-distribution.


#### Step 1. What do we measure, and how is it calculated?

The result is **e_Xav** (Mean relative error of indication). It is not measured directly: it is calculated from the input quantities with the measurement model

```
e_Xav = e_X + de_X
```


#### Step 2. The input quantities and their uncertainties

The model has 2 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **e_X**: Error of a single run (uncertainty from the second budget). Best value 0 with standard uncertainty u = 0.00068132 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **de_X**: Relative errors of the three runs (Type A). Measured 3 times (a **Type A** evaluation, from statistics). The estimate is the mean 0.00100000; the readings scatter with a standard deviation s = 0.00104, and the standard uncertainty of the mean is u = s/√n = 0.00104/√3 = 0.000603, with ν = n − 1 = 2 degrees of freedom.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It chooses the coverage factor k for a coverage probability of 95.5 %, using the effective degrees of freedom (Welch–Satterthwaite), and gives the **expanded uncertainty** U = k·u_c.

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- standard uncertainty u = 0.00091
- coverage factor k = 2.28
- expanded uncertainty U = 0.002

Reference: EA-4/02 M:2022 S12.11–S12.13 (u = 0.91 × 10⁻³, ν_eff = 10, k = 2.28, U = 0.002).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="ea402-s12-water-meter-volume"></a>

### Water meter calibration: reference volume

`ea402-s12-water-meter-volume`

#### What is this project about?

EA-4/02 S12 (first budget): volume passed through a household water meter, measured in a volumetric standard at 2500 L/h.


#### Step 1. What do we measure, and how is it calculated?

The result is **V_X** in L (Volume at the meter). It is not measured directly: it is calculated from the input quantities with the measurement model

```
V_X = (V_iS + dV_iS)*(1 + beta_S*(t_S - 20))*(1 + gamma_W*(t_X - t_S))*(1 - kappa_W*p_X)
```


#### Step 2. The input quantities and their uncertainties

The model has 8 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **V_iS**: Indication of the volumetric standard. Best value 200 L. The source (e.g. a certificate) gives the expanded uncertainty U = 0.2 L with k = 2, so the standard uncertainty is u = U/k = 0.2/2 = 0.100 L (normal distribution).
2. **dV_iS**: Reading of the standard. Only limits are known: the value lies between -0.02 and 0.02 L (0 ± 0.02), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.02/√3 = 0.0115 L.
3. **beta_S**: Cubic expansion coefficient of the standard. Only limits are known: the value lies between 0.0000505 and 0.0000515 1/K (0.000051 ± 5e-7), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 5e-7/√3 = 2.89 × 10⁻⁷ 1/K.
4. **t_S**: Water temperature in the standard. Only limits are known: the value lies between 13 and 17 °C (15 ± 2), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 2/√3 = 1.15 °C.
5. **gamma_W**: Expansion coefficient of water. Only limits are known: the value lies between 0.00014500 and 0.000155 1/K (0.00015 ± 0.000005), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.000005/√3 = 2.89 × 10⁻⁶ 1/K.
6. **t_X**: Water temperature at the meter. Only limits are known: the value lies between 14 and 18 °C (16 ± 2), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 2/√3 = 1.15 °C.
7. **kappa_W**: Compressibility of water. Only limits are known: the value lies between 4.55e-7 and 4.65e-7 1/kPa (4.6e-7 ± 5e-9), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 5e-9/√3 = 2.89 × 10⁻⁹ 1/kPa.
8. **p_X**: Gauge pressure at the meter. Only limits are known: the value lies between 450 and 550 kPa (500 ± 50), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 50/√3 = 28.9 kPa.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 199.93 L
- standard uncertainty u = 0.109 L

Reference: EA-4/02 M:2022 S12.9 (V_X = 199.93 L, u = 0.109 L).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## A resistor in UKAS M3003 (Beginner)

A 10 kΩ resistor calibrated by comparing voltages. A clear example of a budget table with the sources, their values, the divisors (√3, 2, 1) and the resulting standard uncertainties.

<a id="m3003-k1-resistor-10kohm"></a>

### 10 kΩ resistor by voltage intercomparison (UKAS M3003 K.1)

`m3003-k1-resistor-10kohm`

#### What is this project about?

The voltages across a standard and an unknown 10 kΩ resistor in series are compared with one DVM; both resistors are in an oil bath at 20.0 °C ± 0.1 °C.


#### Step 1. What do we measure, and how is it calculated?

The result is **R_X** in Ω (Resistance of the unknown resistor). It is not measured directly: it is calculated from the input quantities with the measurement model

```
R_X = (R_S + dR_D + R_TC*dt) * r * (1 + d_VX) / (1 + d_VS)
```


#### Step 2. The input quantities and their uncertainties

The model has 7 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **R_S**: Standard resistor (certificate, ±0.5 ppm, k = 2). Best value 10000 Ω. The source (e.g. a certificate) gives the expanded uncertainty U = 0.005 Ω with k = 2, so the standard uncertainty is u = U/k = 0.005/2 = 0.00250 Ω (normal distribution).
2. **dR_D**: Drift of the standard since calibration (±0.5 ppm). Only limits are known: the value lies between -0.005 and 0.005 Ω (0 ± 0.005), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.005/√3 = 0.00289 Ω.
3. **R_TC**: Temperature coefficient of R_S (2.5 ppm/°C). An exact value, 0.025 Ω/°C, with no uncertainty (for example a defined constant or a nominal value).
4. **dt**: Deviation of the oil bath from 20.0 °C. Only limits are known: the value lies between -0.1 and 0.1 °C (0 ± 0.1), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.1/√3 = 0.0577 °C.
5. **d_VS**: DVM linearity and resolution, reading across R_S (relative). Only limits are known: the value lies between -2e-7 and 2e-7 (0 ± 2e-7), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 2e-7/√3 = 1.15 × 10⁻⁷.
6. **d_VX**: DVM linearity and resolution, reading across R_X (relative). Only limits are known: the value lies between -2e-7 and 2e-7 (0 ± 2e-7), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 2e-7/√3 = 1.15 × 10⁻⁷.
7. **r**: Voltage ratio V_X/V_S, five readings. Measured 5 times (a **Type A** evaluation, from statistics). The estimate is the mean 1.00001; the readings scatter with a standard deviation s = 1.58 × 10⁻⁷, and the standard uncertainty of the mean is u = s/√n = 1.58 × 10⁻⁷/√5 = 7.07 × 10⁻⁸, with ν = n − 1 = 4 degrees of freedom.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 200 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 200 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 10000.105 Ω
- standard uncertainty u = 0.00445 Ω
- expanded uncertainty U = 0.0089 Ω

Reference: UKAS M3003 ed. 6 K.1.8–K.1.9: u_c = 0.445 ppm, U = 0.891 ppm; 10 000.1050 Ω ± 0.0089 Ω.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## A weighing machine at 200 g (Beginner)

A weighing machine is checked with a 200 g weight. Repeatability, resolution and the weight's certificate are the main terms.

<a id="m3003-k5-weighing-machine-200g"></a>

### Weighing machine of 205 g capacity at 200 g (UKAS M3003 K.5)

`m3003-k5-weighing-machine-200g`

#### What is this project about?

Error of indication of an electronic balance with a 0.1 mg digit, loaded with a class E2 200 g weight; one reading.


#### Step 1. What do we measure, and how is it calculated?

The result is **dI_X** in mg (Error of indication at 200 g). It is not measured directly: it is calculated from the input quantities with the measurement model

```
dI_X = I_X - W_S + dD_S + dI_d0 + dI_d + dA_b + dI_r
```


#### Step 2. The input quantities and their uncertainties

The model has 7 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **I_X**: Indication (199.9999 g). An exact value, 199999.9 mg, with no uncertainty (for example a defined constant or a nominal value).
2. **W_S**: Standard weight (U = 0.1 mg, k = 2). Best value 200000 mg. The source (e.g. a certificate) gives the expanded uncertainty U = 0.1 mg with k = 2, so the standard uncertainty is u = U/k = 0.1/2 = 0.0500 mg (normal distribution).
3. **dD_S**: Drift of the standard. Only limits are known: the value lies between -0.1 and 0.1 mg (0 ± 0.1), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.1/√3 = 0.0577 mg.
4. **dI_d0**: Rounding at zero. Only limits are known: the value lies between -0.05 and 0.05 mg (0 ± 0.05), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.05/√3 = 0.0289 mg.
5. **dI_d**: Rounding of the indication. Only limits are known: the value lies between -0.05 and 0.05 mg (0 ± 0.05), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.05/√3 = 0.0289 mg.
6. **dA_b**: Air buoyancy (±1 ppm). Only limits are known: the value lies between -0.2 and 0.2 mg (0 ± 0.2), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.2/√3 = 0.115 mg.
7. **dI_r**: Repeatability (s = 0.05 mg, ν = 9, one reading). Mean of 1 readings, 0 mg. Instead of the scatter of these few readings, a standard deviation s_p = 0.05 mg determined earlier from more data (ν = 9) is used, so u = s_p/√n = 0.0500 mg.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 200 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 200 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = -0.1 mg
- standard uncertainty u = 0.15275 mg
- expanded uncertainty U = 0.3055 mg

Reference: Recomputed from the contributions of K.5.9 (research log F56). M3003 prints u_c = 0.150 mg, U = 0.300 mg and the result 0.10 mg ± 0.30 mg.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## A pressure indicator at 2 MPa (Beginner)

A digital pressure indicator is compared with a dead-weight tester. A typical pressure calibration budget.

<a id="m3003-k8-pressure-indicator-2mpa"></a>

### Digital pressure indicator at 2 MPa against a dead weight tester (UKAS M3003 K.8)

`m3003-k8-pressure-indicator-2mpa`

#### What is this project about?

Error of a DPI at 2 MPa; the budget is in relative terms (ppm), as is the convention for pressure.


#### Step 1. What do we measure, and how is it calculated?

The result is **e** in ppm (Relative error of indication of the DPI). It is not measured directly: it is calculated from the input quantities with the measurement model

```
e = 1e6*(I_X - P_X)/P_X - (A_cal + a_p + A_drift + m_cal + m_drift + lambda*dt + g + air + head + fluid + level) + d_res + d_rep
```


#### Step 2. The input quantities and their uncertainties

The model has 16 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **I_X**: Indication. An exact value, 2.00084 MPa, with no uncertainty (for example a defined constant or a nominal value).
2. **P_X**: Generated pressure. An exact value, 2.000806 MPa, with no uncertainty (for example a defined constant or a nominal value).
3. **A_cal**: Effective area, calibration (U = 32.2 ppm, k = 2). Best value 0 ppm. The source (e.g. a certificate) gives the expanded uncertainty U = 32.2 ppm with k = 2, so the standard uncertainty is u = U/k = 32.2/2 = 16.1 ppm (normal distribution).
4. **a_p**: Distortion coefficient at 2 MPa (U = 1.0 ppm, k = 2). Best value 0 ppm. The source (e.g. a certificate) gives the expanded uncertainty U = 1 ppm with k = 2, so the standard uncertainty is u = U/k = 1/2 = 0.500 ppm (normal distribution).
5. **A_drift**: Drift of the effective area. Only limits are known: the value lies between -30 and 30 ppm (0 ± 30), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 30/√3 = 17.3 ppm.
6. **m_cal**: Calibration of the total load (U = 7.3 ppm, k = 2). Best value 0 ppm. The source (e.g. a certificate) gives the expanded uncertainty U = 7.3 ppm with k = 2, so the standard uncertainty is u = U/k = 7.3/2 = 3.65 ppm (normal distribution).
7. **m_drift**: Drift of the total load. Only limits are known: the value lies between -7.6 and 7.6 ppm (0 ± 7.6), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 7.6/√3 = 4.39 ppm.
8. **lambda**: Temperature coefficient of piston and cylinder. An exact value, 23 ppm/°C, with no uncertainty (for example a defined constant or a nominal value).
9. **dt**: Temperature of the piston. Only limits are known: the value lies between -0.5 and 0.5 °C (0 ± 0.5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.5/√3 = 0.289 °C.
10. **g**: Local gravity (U = 3 ppm, k = 2). Best value 0 ppm. The source (e.g. a certificate) gives the expanded uncertainty U = 3 ppm with k = 2, so the standard uncertainty is u = U/k = 3/2 = 1.50 ppm (normal distribution).
11. **air**: Air buoyancy approximation. Only limits are known: the value lies between -13 and 13 ppm (0 ± 13), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 13/√3 = 7.51 ppm.
12. **head**: Fluid head (3 mm of oil). Only limits are known: the value lies between -13.5 and 13.5 ppm (0 ± 13.5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 13.5/√3 = 7.79 ppm.
13. **fluid**: Buoyancy volume, surface tension, fluid density. Only limits are known: the value lies between -2 and 2 ppm (0 ± 2), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 2/√3 = 1.15 ppm.
14. **level**: Verticality of the piston (0.15°). Only limits are known: the value lies between -3.4 and 3.4 ppm (0 ± 3.4), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 3.4/√3 = 1.96 ppm.
15. **d_res**: Rounding of the indication (200 Pa steps). Only limits are known: the value lies between -50 and 50 ppm (0 ± 50), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 50/√3 = 28.9 ppm.
16. **d_rep**: Repeatability (s = 16 ppm, ν = 9, one reading). Mean of 1 readings, 0 ppm. Instead of the scatter of these few readings, a standard deviation s_p = 16 ppm determined earlier from more data (ν = 9) is used, so u = s_p/√n = 16.0 ppm.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 200 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 200 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 17 ppm
- standard uncertainty u = 43 ppm
- expanded uncertainty U = 86 ppm

Reference: UKAS M3003 ed. 6 K.8.16–K.8.17: u_c = 43.0 ppm, U = 86.0 ppm; 17 ppm ± 86 ppm.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Thickness of a sheet (Beginner)

The thickness of an aluminium sample is the mean of five readings. It combines a Type A evaluation (the scatter of the readings) with Type B terms: calibration, resolution, drift and temperature.

<a id="m3003-k10-sheet-thickness"></a>

### Thickness of an aluminium test sample (UKAS M3003 K.10)

`m3003-k10-sheet-thickness`

#### What is this project about?

Mean of five thickness readings at the centre of a sample, measured in an uncontrolled environment; the calibration correction is not applied.


#### Step 1. What do we measure, and how is it calculated?

The result is **w** in mm (Thickness at the centre of the sample). It is not measured directly: it is calculated from the input quantities with the measurement model

```
w = w_ave + d_res + d_cal + d_drift + d_unc + c_T*d_T
```


#### Step 2. The input quantities and their uncertainties

The model has 7 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **w_ave**: Indicated thickness, five readings. Measured 5 times (a **Type A** evaluation, from statistics). The estimate is the mean 1.51400 mm; the readings scatter with a standard deviation s = 0.0114 mm, and the standard uncertainty of the mean is u = s/√n = 0.0114/√5 = 0.00510 mm, with ν = n − 1 = 4 degrees of freedom.
2. **d_res**: Resolution. Only limits are known: the value lies between -0.005 and 0.005 mm (0 ± 0.005), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.005/√3 = 0.00289 mm.
3. **d_cal**: Calibration (U = 0.015 mm, k = 2). Best value 0 mm. The source (e.g. a certificate) gives the expanded uncertainty U = 0.015 mm with k = 2, so the standard uncertainty is u = U/k = 0.015/2 = 0.00750 mm (normal distribution).
4. **d_drift**: Calibration drift. Only limits are known: the value lies between -0.00233 and 0.00233 mm (0 ± 0.00233), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.00233/√3 = 0.00135 mm.
5. **d_unc**: Uncorrected calibration error. Only limits are known: the value lies between -0.001 and 0.001 mm (0 ± 0.001), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.001/√3 = 0.000577 mm.
6. **c_T**: Temperature sensitivity. An exact value, 0.0000357 mm/K, with no uncertainty (for example a defined constant or a nominal value).
7. **d_T**: Deviation from the calibration temperature. Only limits are known: the value lies between -10 and 10 K (0 ± 10), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 10/√3 = 5.77 K.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 200 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 200 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 1.514 mm
- standard uncertainty u = 0.00963 mm
- expanded uncertainty U = 0.019 mm

Reference: UKAS M3003 ed. 6 K.10.8–K.10.9: u_c = 0.009 63 mm, ν_eff = 50, U = 0.019 mm; 1.514 mm ± 0.019 mm.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Calibrating volumetric glassware (Beginner)

The volume of a flask or pycnometer is found by weighing the water it holds. You see how the density of water and air, the temperature and the balance enter the result.

<a id="cg19-ex1-flask-1000ml"></a>

### Gravimetric calibration of a 1000 mL volumetric flask (EURAMET cg-19 Example 1)

`cg19-ex1-flask-1000ml`

#### What is this project about?

A 1000 mL glass flask filled to the mark and weighed ten times; water at 19.11 °C, air 0.2 °C warmer.


#### Step 1. What do we measure, and how is it calculated?

The result is **V_20** in mL (Volume at 20 °C). It is not measured directly: it is calculated from the input quantities with the measurement model

```
V_20 = m/(rho_W - rho_A)*(1 - rho_A/rho_B)*(1 - gamma*(t_W + d_ts - 20)) + dV_men + dV_rep
```

To keep the equation readable, 2 intermediate quantities are calculated first:

- `t_W = t_W0 + d_res`
- `rho_W = 0.99997495*(1 - (t_W - 3.983035)^2*(t_W + 301.797)/(522528.9*(t_W + 69.34881))) + d_form + d_pur`


#### Step 2. The input quantities and their uncertainties

The model has 11 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **m**: Mass of water, I_L − I_E (balance U and resolution for both weighings). Best value 996.9505 g with standard uncertainty u = 0.00127 g (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **t_W0**: Measured water temperature (thermometer U = 0.009 °C, k = 2). Best value 19.11 °C. The source (e.g. a certificate) gives the expanded uncertainty U = 0.009 °C with k = 2, so the standard uncertainty is u = U/k = 0.009/2 = 0.00450 °C (normal distribution).
3. **d_res**: Resolution of the thermometer (0.001 °C). Only limits are known: the value lies between -0.0005 and 0.0005 °C (0 ± 0.0005), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.0005/√3 = 0.000289 °C.
4. **d_ts**: Difference between water and instrument temperature. Only limits are known: the value lies between -0.025 and 0.025 °C (0 ± 0.025), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.025/√3 = 0.0144 °C.
5. **d_form**: Uncertainty of Tanaka's formula. Best value 0 g/mL with standard uncertainty u = 4.5e-7 g/mL (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
6. **d_pur**: Purity of the water (5 ppm). Only limits are known: the value lies between -4.9920 × 10⁻⁶ and 4.9920 × 10⁻⁶ g/mL (0 ± 4.9920 × 10⁻⁶), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 4.9920 × 10⁻⁶/√3 = 2.88 × 10⁻⁶ g/mL.
7. **rho_A**: Air density (u as printed in the example). Best value 0.0012 g/mL with standard uncertainty u = 0.00000176 g/mL (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
8. **rho_B**: Density of the reference weights (±0.06 g/mL, u = 0.0346 g/mL as in the summary table). Only limits are known: the value lies between 7.94 and 8.06 g/mL (8 ± 0.06), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.06/√3 = 0.0346 g/mL.
9. **gamma**: Cubic expansion coefficient of the glass (±5 %). Only limits are known: the value lies between 0.0000095 and 1.0500 × 10⁻⁵ 1/°C (0.00001 ± 5e-7), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 5e-7/√3 = 2.89 × 10⁻⁷ 1/°C.
10. **dV_men**: Meniscus reading. Only limits are known: the value lies between -0.015 and 0.015 mL (0 ± 0.015), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.015/√3 = 0.00866 mL.
11. **dV_rep**: Repeatability (s(V₀) = 0.044 mL, mean of 10). Mean of 10 readings, 0 mL. Instead of the scatter of these few readings, a standard deviation s_p = 0.044 mL determined earlier from more data (ν = 9) is used, so u = s_p/√n = 0.0139 mL.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It chooses the coverage factor k for a coverage probability of 95.5 %, using the effective degrees of freedom (Welch–Satterthwaite), and gives the **expanded uncertainty** U = k·u_c.

It also uses the **Monte Carlo method**: it draws 200 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 200 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 999.628 mL
- standard uncertainty u = 0.017 mL
- coverage factor k = 2.15
- expanded uncertainty U = 0.036 mL

Reference: EURAMET cg-19 v4.1 7.1, Table 2: V₂₀ = 999.628 mL, u = 0.017 mL, ν_eff = 19, k = 2.15, U = 0.036 mL. The estimate is compared within 0.01 mL because Table 1 gives the densities rounded (0.9984 and 0.0012 g/mL).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="cg19-ex2-pycnometer-100ml"></a>

### Gravimetric calibration of a 100 mL glass pycnometer (EURAMET cg-19 Example 2)

`cg19-ex2-pycnometer-100ml`

#### What is this project about?

A 100 mL pycnometer weighed ten times; water at 18.94 °C, air 0.23 °C warmer; the meniscus contribution is negligible.


#### Step 1. What do we measure, and how is it calculated?

The result is **V_20** in mL (Volume at 20 °C). It is not measured directly: it is calculated from the input quantities with the measurement model

```
V_20 = m/(rho_W - rho_A)*(1 - rho_A/rho_B)*(1 - gamma*(t_W + d_ts - 20)) + dV_men + dV_rep
```

To keep the equation readable, 2 intermediate quantities are calculated first:

- `t_W = t_W0 + d_res`
- `rho_W = 0.99997495*(1 - (t_W - 3.983035)^2*(t_W + 301.797)/(522528.9*(t_W + 69.34881))) + d_form + d_pur`


#### Step 2. The input quantities and their uncertainties

The model has 11 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **m**: Mass of water, I_L − I_E (balance U and resolution for both weighings). Best value 99.75 g with standard uncertainty u = 0.000114 g (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **t_W0**: Measured water temperature (thermometer U = 0.009 °C, k = 2). Best value 18.94 °C. The source (e.g. a certificate) gives the expanded uncertainty U = 0.009 °C with k = 2, so the standard uncertainty is u = U/k = 0.009/2 = 0.00450 °C (normal distribution).
3. **d_res**: Resolution of the thermometer (0.001 °C). Only limits are known: the value lies between -0.0005 and 0.0005 °C (0 ± 0.0005), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.0005/√3 = 0.000289 °C.
4. **d_ts**: Difference between water and instrument temperature. Only limits are known: the value lies between -0.02875 and 0.02875 °C (0 ± 0.02875), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.02875/√3 = 0.0166 °C.
5. **d_form**: Uncertainty of Tanaka's formula. Best value 0 g/mL with standard uncertainty u = 4.5e-7 g/mL (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
6. **d_pur**: Purity of the water (5 ppm). Only limits are known: the value lies between -4.9920 × 10⁻⁶ and 4.9920 × 10⁻⁶ g/mL (0 ± 4.9920 × 10⁻⁶), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 4.9920 × 10⁻⁶/√3 = 2.88 × 10⁻⁶ g/mL.
7. **rho_A**: Air density (u as printed in the example). Best value 0.0012 g/mL with standard uncertainty u = 0.00000174 g/mL (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
8. **rho_B**: Density of the reference weights (±0.06 g/mL, u = 0.0346 g/mL as in the summary table). Only limits are known: the value lies between 7.94 and 8.06 g/mL (8 ± 0.06), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.06/√3 = 0.0346 g/mL.
9. **gamma**: Cubic expansion coefficient of the glass (±5 %). Only limits are known: the value lies between 0.0000095 and 1.0500 × 10⁻⁵ 1/°C (0.00001 ± 5e-7), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 5e-7/√3 = 2.89 × 10⁻⁷ 1/°C.
10. **dV_men**: Meniscus reading (negligible). An exact value, 0 mL, with no uncertainty (for example a defined constant or a nominal value).
11. **dV_rep**: Repeatability (s(V₀) = 0.0026 mL, mean of 10). Mean of 10 readings, 0 mL. Instead of the scatter of these few readings, a standard deviation s_p = 0.0026 mL determined earlier from more data (ν = 9) is used, so u = s_p/√n = 0.000822 mL.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It chooses the coverage factor k for a coverage probability of 95.5 %, using the effective degrees of freedom (Welch–Satterthwaite), and gives the **expanded uncertainty** U = k·u_c.

It also uses the **Monte Carlo method**: it draws 200 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 200 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 100.014 mL
- standard uncertainty u = 0.00089 mL
- coverage factor k = 2.21
- expanded uncertainty U = 0.002 mL

Reference: EURAMET cg-19 v4.1 7.2, Table 4: V₂₀ = 100.014 mL, u = 0.89 µL, ν_eff = 13 (7.2.5), k = 2.21, U = 2.0 µL. ν_eff = 12.9 is rounded to 13 in the example; GUM G.6.4 truncates it to 12, which gives k = 2.23 and the same U = 2.0 µL.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Preparing a calibration standard (Beginner)

A cadmium solution is made by dissolving a weighed amount of pure metal in a known volume. The classic first example of the Eurachem guide: purity, mass and volume combine as relative uncertainties because the model is a product and a ratio.

<a id="quam-a1-cadmium-standard-volume"></a>

### Cadmium standard with the volume sub-model

`quam-a1-cadmium-standard-volume`

#### What is this project about?

QUAM Example A1 with the volume of the flask modelled from its three components.


#### Step 1. What do we measure, and how is it calculated?

The result is **c_Cd** in mg/L (Concentration of the calibration standard). It is not measured directly: it is calculated from the input quantities with the measurement model

```
c_Cd = 1000 * m * P / V
```

To keep the equation readable, 1 intermediate quantities are calculated first:

- `V = V_nom + dV_cal + dV_rep + dV_temp`


#### Step 2. The input quantities and their uncertainties

The model has 6 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **P**: Purity of the cadmium metal (certificate: 99.99 ± 0.01 %). Only limits are known: the value lies between 0.9998 and 1 (0.9999 ± 0.0001), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.0001/√3 = 5.77 × 10⁻⁵.
2. **m**: Mass of the cadmium (weighing by difference). Best value 100.28 mg with standard uncertainty u = 0.05 mg (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **V_nom**: Nominal volume. An exact value, 100 mL, with no uncertainty (for example a defined constant or a nominal value).
4. **dV_cal**: Flask calibration (±0.1 mL). The value lies between -0.1 and 0.1 mL, and values near the centre 0 are more likely than values near the limits (triangular distribution). u = a/√6 = 0.1/√6 = 0.0408 mL.
5. **dV_rep**: Filling repeatability. Best value 0 mL with standard uncertainty u = 0.02 mL (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
6. **dV_temp**: Temperature effect (±4 °C). Only limits are known: the value lies between -0.084 and 0.084 mL (0 ± 0.084), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.084/√3 = 0.0485 mL.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 2 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 2 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 1002.7 mg/L
- expanded uncertainty U = 1.7 mg/L

Reference: QUAM:2012 A1.4: u(V) = 0.066 mL; U = 2u_c = 1.7 mg/L before the guide's rounding of u_c to 0.9 mg/L.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="quam-a1-cadmium-standard"></a>

### Preparation of a cadmium calibration standard

`quam-a1-cadmium-standard`

#### What is this project about?

EURACHEM/CITAC QUAM Example A1: 1000 mg/L Cd standard from high-purity metal (summary budget).


#### Step 1. What do we measure, and how is it calculated?

The result is **c_Cd** in mg/L (Concentration of the calibration standard). It is not measured directly: it is calculated from the input quantities with the measurement model

```
c_Cd = 1000 * m * P / V
```


#### Step 2. The input quantities and their uncertainties

The model has 3 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **P**: Purity of the cadmium metal (certificate: 99.99 ± 0.01 %). Only limits are known: the value lies between 0.9998 and 1 (0.9999 ± 0.0001), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.0001/√3 = 5.77 × 10⁻⁵.
2. **m**: Mass of the cadmium (weighing by difference). Best value 100.28 mg with standard uncertainty u = 0.05 mg (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **V**: Volume of the flask. Best value 100 mL with standard uncertainty u = 0.07 mL (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 1002.7 mg/L
- standard uncertainty u = 0.8633 mg/L

Reference: QUAM:2012 Table A1.3 (spreadsheet value u = 0.8633 mg/L).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Bias from a reference material or proficiency tests (Beginner)

The top-down way of finding the systematic part of the uncertainty: measure a certified reference material, or look at your results in proficiency tests, and turn the differences into u(bias). Compare the variants with one CRM, several CRMs, PT results and recovery.

<a id="nordtest-6.1-one-crm"></a>

### Bias from one certified reference material

`nordtest-6.1-one-crm`

#### What is this project about?

Nordtest TR 537 §6.1: u(bias) from a CRM (11.5 ± 0.5) measured 12 times.


#### Step 1. The idea: uncertainty from the laboratory's own data

This is a **top-down** evaluation of **c**. Instead of modelling every step of the method, it uses data that the laboratory already has: how much the results scatter when the method is repeated on different days (precision) and how far they are, on average, from the true value (bias or recovery). Together these cover most sources of uncertainty at once.


#### Step 2. The data

**Bias** (systematic effects): 1 source, e.g. certified reference materials or proficiency tests. From the differences between the laboratory's results and the reference values, the program estimates u(bias), which includes the uncertainty of the reference values themselves.


#### Step 3. What the program calculates

It combines the components as u = √(u(precision)² + u(bias)² + …), at each level, and multiplies by the coverage factor (k = 2 for about 95 %) to get the expanded uncertainty U. The results show u and U, absolute and relative, at every level.


> **Try:** Compare the size of the precision and bias components in the results. If bias dominates, a better reference material or a correction helps more than repeating measurements.


#### Finally: what the source expects

Reference: Nordtest TR 537 §6.1, recomputed (the handbook prints 4.1 %, see notes).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="nordtest-6.1-three-crms"></a>

### Bias from several certified reference materials

`nordtest-6.1-three-crms`

#### What is this project about?

Nordtest TR 537 §6.1: u(bias) from three CRMs by the RMS of the biases.


#### Step 1. The idea: uncertainty from the laboratory's own data

This is a **top-down** evaluation of **c**. Instead of modelling every step of the method, it uses data that the laboratory already has: how much the results scatter when the method is repeated on different days (precision) and how far they are, on average, from the true value (bias or recovery). Together these cover most sources of uncertainty at once.


#### Step 2. The data

**Bias** (systematic effects): 1 source, e.g. certified reference materials or proficiency tests. From the differences between the laboratory's results and the reference values, the program estimates u(bias), which includes the uncertainty of the reference values themselves.


#### Step 3. What the program calculates

It combines the components as u = √(u(precision)² + u(bias)² + …), at each level, and multiplies by the coverage factor (k = 2 for about 95 %) to get the expanded uncertainty U. The results show u and U, absolute and relative, at every level.


> **Try:** Compare the size of the precision and bias components in the results. If bias dominates, a better reference material or a correction helps more than repeating measurements.


#### Finally: what the source expects

Reference: Nordtest TR 537 §6.1 (RMS_bias 2.53 %, ū(C_ref) 1.92 %, u(bias) 3.2 %).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="nordtest-6.2-proficiency-tests"></a>

### Bias from proficiency tests

`nordtest-6.2-proficiency-tests`

#### What is this project about?

Nordtest TR 537 §6.2: u(bias) from six proficiency tests.


#### Step 1. The idea: uncertainty from the laboratory's own data

This is a **top-down** evaluation of **c**. Instead of modelling every step of the method, it uses data that the laboratory already has: how much the results scatter when the method is repeated on different days (precision) and how far they are, on average, from the true value (bias or recovery). Together these cover most sources of uncertainty at once.


#### Step 2. The data

**Bias** (systematic effects): 1 source, e.g. certified reference materials or proficiency tests. From the differences between the laboratory's results and the reference values, the program estimates u(bias), which includes the uncertainty of the reference values themselves.


#### Step 3. What the program calculates

It combines the components as u = √(u(precision)² + u(bias)² + …), at each level, and multiplies by the coverage factor (k = 2 for about 95 %) to get the expanded uncertainty U. The results show u and U, absolute and relative, at every level.


> **Try:** Compare the size of the precision and bias components in the results. If bias dominates, a better reference material or a correction helps more than repeating measurements.


#### Finally: what the source expects

Reference: Nordtest TR 537 §6.2 (RMS_bias 4.6 %, ū(C_ref) 2.6 %, u(bias) 5.3 %).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="nordtest-6.3-recovery"></a>

### Bias from recovery experiments

`nordtest-6.3-recovery`

#### What is this project about?

Nordtest TR 537 §6.3: u(bias) from six spike recoveries.


#### Step 1. The idea: uncertainty from the laboratory's own data

This is a **top-down** evaluation of **c**. Instead of modelling every step of the method, it uses data that the laboratory already has: how much the results scatter when the method is repeated on different days (precision) and how far they are, on average, from the true value (bias or recovery). Together these cover most sources of uncertainty at once.


#### Step 2. The data

**Bias** (systematic effects): 1 source, e.g. certified reference materials or proficiency tests. From the differences between the laboratory's results and the reference values, the program estimates u(bias), which includes the uncertainty of the reference values themselves.


#### Step 3. What the program calculates

It combines the components as u = √(u(precision)² + u(bias)² + …), at each level, and multiplies by the coverage factor (k = 2 for about 95 %) to get the expanded uncertainty U. The results show u and U, absolute and relative, at every level.


> **Try:** Compare the size of the precision and bias components in the results. If bias dominates, a better reference material or a correction helps more than repeating measurements.


#### Finally: what the source expects

Reference: Nordtest TR 537 §6.3 (RMS_bias 3.44 %, u(C_recovery) 1.0 %, u(bias) 3.6 %).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Precision from duplicates (Beginner)

Analysing samples in duplicate gives the repeatability; a control sample measured over months gives the within-laboratory reproducibility. These are the random part of a top-down uncertainty.

<a id="nordtest-5.3-ammonium-duplicates"></a>

### Ammonium: within-laboratory reproducibility from control samples and duplicates

`nordtest-5.3-ammonium-duplicates`

#### What is this project about?

Nordtest TR 537 §5.3 and Appendix 5: NH₄-N duplicates of natural samples in two ranges (raw data), combined with the control-sample reproducibility.


#### Step 1. The idea: uncertainty from the laboratory's own data

This is a **top-down** evaluation of **c(NH₄-N)** (µg/L). Instead of modelling every step of the method, it uses data that the laboratory already has: how much the results scatter when the method is repeated on different days (precision) and how far they are, on average, from the true value (bias or recovery). Together these cover most sources of uncertainty at once.

The uncertainty is evaluated at 2 concentration levels: 10; 100 µg/L.


#### Step 2. The data

**Precision** (random effects), 4 entries:

- **u(Rw), control < 30**: a standard deviation of 0.5 (e.g. from a control chart: within-laboratory reproducibility).
- **s_r, duplicates < 30**: 46 duplicate pairs; the standard deviation follows from the differences within the pairs.
- **u(Rw), control ≥ 30**: a standard deviation of 1.5 % (e.g. from a control chart: within-laboratory reproducibility).
- **s_r, duplicates ≥ 30**: 26 duplicate pairs; the standard deviation follows from the differences within the pairs.


#### Step 3. What the program calculates

It combines the components as u = √(u(precision)² + u(bias)² + …), at each level, and multiplies by the coverage factor (k = 2 for about 95 %) to get the expanded uncertainty U. The results show u and U, absolute and relative, at every level.


> **Try:** Compare the size of the precision and bias components in the results. If bias dominates, a better reference material or a correction helps more than repeating measurements.


#### Finally: what the source expects

Reference: Nordtest TR 537 §5.3 and Appendix 5 (s_r 0.44 µg/L and 3.8 %; u(Rw) 0.7 µg/L and 4.1 %).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="nordtest-5.3-oxygen-duplicates"></a>

### Dissolved oxygen: reproducibility from duplicates

`nordtest-5.3-oxygen-duplicates`

#### What is this project about?

Nordtest TR 537 §5.3 and Appendix 6: dissolved oxygen in sea water, 51 duplicate pairs (raw data) and a control-sample component.


#### Step 1. The idea: uncertainty from the laboratory's own data

This is a **top-down** evaluation of **c(O₂)** (mg/L). Instead of modelling every step of the method, it uses data that the laboratory already has: how much the results scatter when the method is repeated on different days (precision) and how far they are, on average, from the true value (bias or recovery). Together these cover most sources of uncertainty at once.


#### Step 2. The data

**Precision** (random effects), 2 entries:

- **s_r, duplicates**: 51 duplicate pairs; the standard deviation follows from the differences within the pairs.
- **u(Rw), control**: a standard deviation of 0.5 % (e.g. from a control chart: within-laboratory reproducibility).


#### Step 3. What the program calculates

It combines the components as u = √(u(precision)² + u(bias)² + …), at each level, and multiplies by the coverage factor (k = 2 for about 95 %) to get the expanded uncertainty U. The results show u and U, absolute and relative, at every level.


> **Try:** Compare the size of the precision and bias components in the results. If bias dominates, a better reference material or a correction helps more than repeating measurements.


#### Finally: what the source expects

Reference: Nordtest TR 537 §5.3 and Appendix 6 (s = 0.0252 mg/L, 0.34 %; u(Rw) = 0.60 %).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## The whole top-down budget in one example (Beginner)

Ammonium in water: precision from the control chart and bias from proficiency tests are combined into the expanded uncertainty. This is the recipe that most testing laboratories use.

<a id="nordtest-3-ammonium-pt"></a>

### Ammonium in water from control limits and proficiency tests

`nordtest-3-ammonium-pt`

#### What is this project about?

Nordtest TR 537 §3.2 and Appendix 4: NH₄-N, u(Rw) from the control chart and u(bias) from six proficiency tests (raw PT data).


#### Step 1. The idea: uncertainty from the laboratory's own data

This is a **top-down** evaluation of **c(NH₄-N)** (µg/L). Instead of modelling every step of the method, it uses data that the laboratory already has: how much the results scatter when the method is repeated on different days (precision) and how far they are, on average, from the true value (bias or recovery). Together these cover most sources of uncertainty at once.


#### Step 2. The data

**Precision** (random effects), 1 entry:

- **u(Rw)**: control_limits

**Bias** (systematic effects): 1 source, e.g. certified reference materials or proficiency tests. From the differences between the laboratory's results and the reference values, the program estimates u(bias), which includes the uncertainty of the reference values themselves.


#### Step 3. What the program calculates

It combines the components as u = √(u(precision)² + u(bias)² + …), at each level, and multiplies by the coverage factor (k = 2 for about 95 %) to get the expanded uncertainty U. The results show u and U, absolute and relative, at every level.


> **Try:** Compare the size of the precision and bias components in the results. If bias dominates, a better reference material or a correction helps more than repeating measurements.


#### Finally: what the source expects

Reference: Nordtest TR 537 §3.2 and Appendix 4 (RMS_bias 2.26 %, ū(C_ref) 1.52 %, u(bias) 2.73 %, u_c 3.20 %, U 6.4 → 7 %).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## A first control chart (Beginner)

Sixty control values of zinc in water are plotted with their warning and action limits. Learn to read a chart: which points are fine, which ones are warnings and which ones demand action.

<a id="tr569-table1-zinc"></a>

### Zinc in water: X-chart from 60 control values (Nordtest TR 569 Table 1)

`tr569-table1-zinc`

#### What is this project about?

Sixty control values of a 60.0 µg/L zinc solution (Table 1), plotted in Figure 1. Statistical limits from the values themselves; daily interpretation of every run.


#### Step 1. The idea: watching a method over time

A **control chart** (Nordtest TR 569) plots the result of a control sample, measured with every series, against time. Warning limits (±2 s) and action limits (±3 s) around a central line show when the method is no longer in statistical control.

60 control values are entered.


> **Try:** In the results, look for points beyond the action limits and for two of three successive points beyond the warning limits: these are the rules that say "stop and find the cause".


#### Finally: what the source expects

Reference: Values of Table 1; mean, s and limits computed from them (TR 569 prints the chart, Figure 1, without the numbers); the zones of Figure 1 agree.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Setting the limits of a control chart (Beginner)

The three ways of setting limits: from the data (statistical), from a target, or around a reference value. Compare the variants to see why target limits are often wider and more useful.

<a id="tr569-ch7-case1-statistical"></a>

### Setting the limits of an X-chart, case 1 (Nordtest TR 569 Chapter 7)

`tr569-ch7-case1-statistical`

#### What is this project about?

Case 1: statistical limits and a mean central line, s_Rw = 4 % of 59.2 µg/L.


#### Step 1. The idea: watching a method over time

A **control chart** (Nordtest TR 569) plots the result of a control sample, measured with every series, against time. Warning limits (±2 s) and action limits (±3 s) around a central line show when the method is no longer in statistical control.

Limits: statistical, from the scatter of the control values themselves.


> **Try:** In the results, look for points beyond the action limits and for two of three successive points beyond the warning limits: these are the rules that say "stop and find the cause".


#### Finally: what the source expects

Reference: TR 569 Chapter 7, values to 0.1 µg/L.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="tr569-ch7-case2-target-mean"></a>

### Setting the limits of an X-chart, case 2 (Nordtest TR 569 Chapter 7)

`tr569-ch7-case2-target-mean`

#### What is this project about?

Case 2: target limits from a required s_Rw of 5 %, mean central line 59.2 µg/L.


#### Step 1. The idea: watching a method over time

A **control chart** (Nordtest TR 569) plots the result of a control sample, measured with every series, against time. Warning limits (±2 s) and action limits (±3 s) around a central line show when the method is no longer in statistical control.

Limits: target limits, from the precision the method must reach.


> **Try:** In the results, look for points beyond the action limits and for two of three successive points beyond the warning limits: these are the rules that say "stop and find the cause".


#### Finally: what the source expects

Reference: TR 569 Chapter 7, values to 0.1 µg/L.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="tr569-ch7-case3-target-reference"></a>

### Setting the limits of an X-chart, case 3 (Nordtest TR 569 Chapter 7)

`tr569-ch7-case3-target-reference`

#### What is this project about?

Case 3: target limits of 5 % around the reference value 60.0 µg/L.


#### Step 1. The idea: watching a method over time

A **control chart** (Nordtest TR 569) plots the result of a control sample, measured with every series, against time. Warning limits (±2 s) and action limits (±3 s) around a central line show when the method is no longer in statistical control.

Limits: target limits, from the precision the method must reach.


> **Try:** In the results, look for points beyond the action limits and for two of three successive points beyond the warning limits: these are the rules that say "stop and find the cause".


#### Finally: what the source expects

Reference: TR 569 Chapter 7, values to 0.1 µg/L.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="tr569-ch7-range-case1-statistical"></a>

### Statistical limits of an r%-chart for duplicates (Nordtest TR 569 Chapter 7)

`tr569-ch7-range-case1-statistical`

#### What is this project about?

Case 1 for range charts: pooled relative standard deviation 0.356 % from duplicates.


#### Step 1. The idea: watching a method over time

A **control chart** (Nordtest TR 569) plots the result of a control sample, measured with every series, against time. Warning limits (±2 s) and action limits (±3 s) around a central line show when the method is no longer in statistical control.

Limits: statistical, from the scatter of the control values themselves.


> **Try:** In the results, look for points beyond the action limits and for two of three successive points beyond the warning limits: these are the rules that say "stop and find the cause".


#### Finally: what the source expects

Reference: TR 569 Chapter 7, case 1, values to 0.1 %. The text prints the warning limit as 3·0.356 = 1.0 %; the factor of Table 4 is 2.83 (research log F52). The central line is not given; the program uses the expected range d₂·s = 0.40 %.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="tr569-ch7-range-case2-target"></a>

### Target limits of an r%-chart from a repeatability limit (Nordtest TR 569 Chapter 7)

`tr569-ch7-range-case2-target`

#### What is this project about?

Case 2 for range charts: the standard method gives a repeatability limit r = 1 %.


#### Step 1. The idea: watching a method over time

A **control chart** (Nordtest TR 569) plots the result of a control sample, measured with every series, against time. Warning limits (±2 s) and action limits (±3 s) around a central line show when the method is no longer in statistical control.

Limits: target limits, from the precision the method must reach.


> **Try:** In the results, look for points beyond the action limits and for two of three successive points beyond the warning limits: these are the rules that say "stop and find the cause".


#### Finally: what the source expects

Reference: TR 569 Chapter 7, case 2.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Probability that an item conforms (Beginner)

A measured value lies inside the specification, but how sure can we be? These examples calculate the probability of conformity from the measured value and its uncertainty, for one-sided and two-sided limits.

<a id="jcgm106-7.3-bursting-strength"></a>

### Bursting strength of a container: probability of conformity (JCGM 106 7.3)

`jcgm106-7.3-bursting-strength`

#### What is this project about?

JCGM 106:2012 7.3 Example 2: single lower tolerance limit B ≥ 490 kPa, destructive test b = 509.7 kPa with u = 8.6 kPa.


#### Step 1. The idea: does the item meet the specification?

The measured quantity B must be at least 490 kPa. Because every measured value is uncertain, a result close to a limit could be on either side of it. The program calculates the **probability of conformity** and applies a **decision rule** (JCGM 106).


#### Finally: what the source expects

Reference: JCGM 106:2012 7.3, Example 2 (p_c = 0.99).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="jcgm106-7.3-zener-diode"></a>

### Breakdown voltage of a Zener diode: probability of conformity (JCGM 106 7.3)

`jcgm106-7.3-zener-diode`

#### What is this project about?

JCGM 106:2012 7.3 Example 1: single upper tolerance limit V_b ≤ −5.40 V, measured −5.47 V with u = 0.05 V.


#### Step 1. The idea: does the item meet the specification?

The measured quantity V_b must be at most -5.4 V. Because every measured value is uncertain, a result close to a limit could be on either side of it. The program calculates the **probability of conformity** and applies a **decision rule** (JCGM 106).


#### Finally: what the source expects

Reference: JCGM 106:2012 7.3, Example 1 (p_c = 0.92).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="jcgm106-7.4-motor-oil-viscosity"></a>

### Kinematic viscosity of a motor oil: two-sided tolerance (JCGM 106 7.4)

`jcgm106-7.4-motor-oil-viscosity`

#### What is this project about?

JCGM 106:2012 7.4 Example: SAE grade 40 oil at 100 °C, 12.5 mm²/s ≤ Y ≤ 16.3 mm²/s, measured 13.6 mm²/s with u = 1.8 mm²/s.


#### Step 1. The idea: does the item meet the specification?

The measured quantity Y must be between 12.5 and 16.3 mm²/s. Because every measured value is uncertain, a result close to a limit could be on either side of it. The program calculates the **probability of conformity** and applies a **decision rule** (JCGM 106).


#### Finally: what the source expects

Reference: JCGM 106:2012 7.4, Example (p_c = 0.66); C_m from Formula (12).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## The assigned value of a proficiency test (Beginner)

The same results evaluated with two robust estimators: Algorithm A and the median. Robust statistics keep a few outlying laboratories from moving the assigned value.

<a id="iso13528-e3-atrazine-algorithm-a"></a>

### Atrazine in drinking water: robust average by Algorithm A (ISO 13528 E.3)

`iso13528-e3-atrazine-algorithm-a`

#### What is this project about?

ISO 13528:2022 E.3: assigned value and standard deviation for proficiency assessment from 34 participant results with Algorithm A; z scores.


#### Step 1. The idea: comparing laboratories

In a **proficiency test**, the same material is sent to 34 laboratories, which all measure **atrazine**. Each laboratory then gets a score that tells whether its result agrees with the assigned value.

Assigned value: the robust average of the participants' results (Algorithm A, which limits the influence of outliers). The standard deviation for proficiency assessment σ_pt decides how far a result may be from the assigned value.


#### Step 2. The scores

- **z = (x − x_pt)/σ_pt**: |z| ≤ 2 satisfactory, 2 < |z| < 3 questionable, |z| ≥ 3 unsatisfactory.
- **ζ and E_n** also use the uncertainty that each laboratory reported; they check whether the laboratory's own uncertainty is realistic.


#### Finally: what the source expects

Reference: ISO 13528:2022 Table E.4 (x* = 0.2570, s* = 0.0395 mg/L after 6 iterations) and Table E.5 (median 0.2620, nIQR 0.0402, MADe 0.0386, u(x_pt) = 0.0085, arithmetic 0.2512 and 0.0672 mg/L); z scores recomputed from these values.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="iso13528-e3-atrazine-median"></a>

### Atrazine in drinking water: median and nIQR (ISO 13528 E.3)

`iso13528-e3-atrazine-median`

#### What is this project about?

ISO 13528:2022 E.3, first row of Table E.5: the median as assigned value with u(x_pt) = 1.25 nIQR/√p and the nIQR as σ_pt.


#### Step 1. The idea: comparing laboratories

In a **proficiency test**, the same material is sent to 34 laboratories, which all measure **atrazine**. Each laboratory then gets a score that tells whether its result agrees with the assigned value.

Assigned value: the median of the participants' results. The standard deviation for proficiency assessment σ_pt decides how far a result may be from the assigned value.


#### Step 2. The scores

- **z = (x − x_pt)/σ_pt**: |z| ≤ 2 satisfactory, 2 < |z| < 3 questionable, |z| ≥ 3 unsatisfactory.
- **ζ and E_n** also use the uncertainty that each laboratory reported; they check whether the laboratory's own uncertainty is realistic.


#### Finally: what the source expects

Reference: ISO 13528:2022 Table E.5, row 'Robust: Median, nIQR (MADe)': 0.2620, 0.0402 (0.0386), u(x_pt) = 0.0086 mg/L.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Scoring laboratories: z, ζ and E_n (Beginner)

Every participant gets a z-score (is the result close enough to the assigned value?) and ζ and E_n scores (is the result consistent with the uncertainty the laboratory claimed?). The table of the standard is reproduced for all 21 numerical results.

<a id="iso13528-e4-mercury-feed"></a>

### Mercury in animal feed: z, z′, ζ and E_n scores (ISO 13528 E.4)

`iso13528-e4-mercury-feed`

#### What is this project about?

ISO 13528:2022 E.4 (IMEP 111): scores of 21 laboratories against a reference value with a fitness-for-purpose σ_pt, using the uncertainties the laboratories reported.


#### Step 1. The idea: comparing laboratories

In a **proficiency test**, the same material is sent to 24 laboratories, which all measure **total Hg**. Each laboratory then gets a score that tells whether its result agrees with the assigned value.

Assigned value: a reference value determined independently (e.g. by a reference laboratory or from a certified reference material). The standard deviation for proficiency assessment σ_pt decides how far a result may be from the assigned value.


#### Step 2. The scores

- **z = (x − x_pt)/σ_pt**: |z| ≤ 2 satisfactory, 2 < |z| < 3 questionable, |z| ≥ 3 unsatisfactory.
- **ζ and E_n** also use the uncertainty that each laboratory reported; they check whether the laboratory's own uncertainty is realistic.


#### Finally: what the source expects

Reference: ISO 13528:2022 Table E.7 (D %, PA, z, z′, ζ and E_n of the 21 numerical results; tolerances half a unit of the printed digit). Robust x* and s* from E.7.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## A sum of four concentrations (Beginner)

The plasma anion gap is a linear combination of four ion concentrations. With a linear model GUM and Monte Carlo agree perfectly: a good check that you understand the basics.

<a id="possolo-meija-2022-plasma-anion-gap"></a>

### Plasma anion gap: a linear combination of four ion concentrations (Possolo & Meija 2022)

`possolo-meija-2022-plasma-anion-gap`

#### What is this project about?

Δc_AG = c(Na⁺) + c(K⁺) − c(Cl⁻) − c(HCO₃⁻) for one patient (White 2008); the uncertainties are reported as standard deviations.


#### Step 1. What do we measure, and how is it calculated?

The result is **AG** in mmol/L (Plasma anion gap Δc_AG). It is not measured directly: it is calculated from the input quantities with the measurement model

```
AG = c_Na + c_K - c_Cl - c_HCO3
```


#### Step 2. The input quantities and their uncertainties

The model has 4 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **c_Na**: Amount concentration of Na⁺. Best value 137 mmol/L with standard uncertainty u = 1.48 mmol/L (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **c_K**: Amount concentration of K⁺. Best value 4 mmol/L with standard uncertainty u = 0.04 mmol/L (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **c_Cl**: Amount concentration of Cl⁻. Best value 106 mmol/L with standard uncertainty u = 0.72 mmol/L (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **c_HCO3**: Amount concentration of HCO₃⁻. Best value 10 mmol/L with standard uncertainty u = 0.84 mmol/L (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 25 mmol/L
- standard uncertainty u = 1.85 mmol/L

Reference: Possolo & Meija (2022), p. 17: Δc_AG = 25 mmol/L, u = 1.85 mmol/L.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Hardness of water (Beginner)

Total hardness from the calcium and magnesium concentrations: a short, conventional measurement model from the IUPAC guide, ideal for practising the steps of the GUM.

<a id="iupac2024-4a-water-hardness"></a>

### Total hardness of water from Ca²⁺ and Mg²⁺: a conventional measurement model (IUPAC 2024, Examples 3C and 4A)

`iupac2024-4a-water-hardness`

#### What is this project about?

H = γ(Ca²⁺)·M(CaCO₃)/M(Ca) + γ(Mg²⁺)·M(CaCO₃)/M(Mg), defined by convention; Gauss's formula with independent inputs.


#### Step 1. What do we measure, and how is it calculated?

The result is **H** in mg/L (Total hardness of water as mass concentration of CaCO₃). It is not measured directly: it is calculated from the input quantities with the measurement model

```
H = g_Ca*M_CaCO3/M_Ca + g_Mg*M_CaCO3/M_Mg
```


#### Step 2. The input quantities and their uncertainties

The model has 5 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **g_Ca**: Mass concentration of Ca²⁺ (AAS). Best value 23.2 mg/L with standard uncertainty u = 0.8 mg/L (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **g_Mg**: Mass concentration of Mg²⁺ (AAS, measured separately). Best value 9.4 mg/L with standard uncertainty u = 0.6 mg/L (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **M_CaCO3**. An exact value, 100.0869 g/mol, with no uncertainty (for example a defined constant or a nominal value).
4. **M_Ca**. An exact value, 40.078 g/mol, with no uncertainty (for example a defined constant or a nominal value).
5. **M_Mg**. An exact value, 24.305 g/mol, with no uncertainty (for example a defined constant or a nominal value).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 96.65 mg/L
- standard uncertainty u = 3 mg/L

Reference: IUPAC 2024, Example 4A: u²(H) = 2.5² × 0.8² + 4.1² × 0.6² ≈ (3 mg/L)².

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## A titration (Beginner)

The concentration of sodium chloride by titration with silver nitrate. Volumes, the concentration of the titrant and the endpoint are the inputs.

<a id="iupac2024-4i-argentometric-titration"></a>

### Amount concentration of NaCl by argentometric titration (Mohr): conventional model (IUPAC 2024, Example 4I)

`iupac2024-4i-argentometric-titration`

#### What is this project about?

c(NaCl) = c(AgNO₃)·(V(AgNO₃) + ΔV_V + ΔV_E)/(V(NaCl) + ΔV_V); only the volumetric and endpoint uncertainties are considered.


#### Step 1. What do we measure, and how is it calculated?

The result is **c_NaCl** in mmol/L (Amount concentration of NaCl). It is not measured directly: it is calculated from the input quantities with the measurement model

```
c_NaCl = c_Ag*(V_Ag + dV_Ag + dV_E)/(V_NaCl + dV_NaCl)
```


#### Step 2. The input quantities and their uncertainties

The model has 6 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **c_Ag**: Concentration of the AgNO₃ titrant (taken as exact). An exact value, 100 mmol/L, with no uncertainty (for example a defined constant or a nominal value).
2. **V_Ag**: Volume of titrant read at the endpoint. An exact value, 10 mL, with no uncertainty (for example a defined constant or a nominal value).
3. **dV_Ag**: Resolution of the electronic burette, ±0.01 mL. The value lies between -0.01 and 0.01 mL, and values near the centre 0 are more likely than values near the limits (triangular distribution). u = a/√6 = 0.01/√6 = 0.00408 mL.
4. **dV_E**: Visual endpoint detection, one droplet at 95 %. Best value 0 mL with standard uncertainty u = 0.025 mL (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
5. **V_NaCl**: Nominal volume of the sample. An exact value, 100 mL, with no uncertainty (for example a defined constant or a nominal value).
6. **dV_NaCl**: Class A volumetric pipette, triangular with base 0.16 mL. The value lies between -0.08 and 0.08 mL, and values near the centre 0 are more likely than values near the limits (triangular distribution). u = a/√6 = 0.08/√6 = 0.0327 mL.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 10 mmol/L
- standard uncertainty u = 0.026 mmol/L

Reference: IUPAC 2024, Example 4I: c(NaCl) = 10.000 mmol/L, u = 0.026 mmol/L (0.26 %).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Molar mass from atomic weights (Beginner)

The relative molecular mass of glucose from the standard atomic weights, which are given as intervals. An interval with no other information becomes a rectangular distribution.

<a id="gum5-e3-molar-mass-glucose"></a>

### Relative molecular mass of glucose from standard atomic weight intervals (JCGM GUM-5 Example 3)

`gum5-e3-molar-mass-glucose`

#### What is this project about?

M_r(C₆H₁₂O₆) = 6A_C + 12A_H + 6A_O with rectangular distributions over the IUPAC 2021 intervals of the standard atomic weights.


#### Step 1. What do we measure, and how is it calculated?

The result is **M_r** (Relative molecular mass of glucose). It is not measured directly: it is calculated from the input quantities with the measurement model

```
M_r = 6*A_C + 12*A_H + 6*A_O
```


#### Step 2. The input quantities and their uncertainties

The model has 3 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **A_H**: Standard atomic weight of hydrogen. Only limits are known: the value lies between 1.00784 and 1.00811 (1.007975 ± 0.00013500), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.00013500/√3 = 7.79 × 10⁻⁵.
2. **A_C**: Standard atomic weight of carbon. Only limits are known: the value lies between 12.0096 and 12.0116 (12.0106 ± 0.0010000), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.0010000/√3 = 0.000577.
3. **A_O**: Standard atomic weight of oxygen. Only limits are known: the value lies between 15.99903 and 15.99977 (15.9994 ± 0.00037000), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.00037000/√3 = 0.000214.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 180.1557
- standard uncertainty u = 0.0038

Reference: JCGM GUM-5:2026, 3.3.3 and 3.3.4: M_r = 180.1557, u = 0.0038.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Radioactive decay (Beginner)

The activity of a source is corrected for the time since its reference date. The half-life and the time enter through an exponential function.

<a id="gum5-e14-radioactive-decay-bi207"></a>

### Activity of a ²⁰⁷Bi source corrected for decay (JCGM GUM-5 Example 14)

`gum5-e14-radioactive-decay-bi207`

#### What is this project about?

A₁ = A₀ exp(−Δt ln2/T½) with the certified activity, the half-life from the literature and Δt = 2.73 a; check of the linearization.


#### Step 1. What do we measure, and how is it calculated?

The result is **A_1** in kBq (Activity at the time of use). It is not measured directly: it is calculated from the input quantities with the measurement model

```
A_1 = A_0*exp(-dt*ln(2)/T_half)
```


#### Step 2. The input quantities and their uncertainties

The model has 3 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **A_0**: Activity at the reference date (certificate). Best value 11.45 kBq with standard uncertainty u = 0.05 kBq (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **T_half**: Half-life of ²⁰⁷Bi. Best value 32.9 a with standard uncertainty u = 1.4 a (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **dt**: Decay time t₁ − t₀. An exact value, 2.73 a, with no uncertainty (for example a defined constant or a nominal value).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 10.81 kBq
- standard uncertainty u = 0.054 kBq

Reference: JCGM GUM-5:2026, 14.3.2: A₁ = 10.810 kBq, u(A₁) = 0.054 kBq (contributions 0.0472 and 0.0265 kBq).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Mean of daily temperatures (Beginner)

The monthly mean of daily maximum temperatures: a pure Type A evaluation. With few days, the t distribution makes the coverage factor clearly larger than 2.

<a id="nist-tn1900-e2-surface-temperature"></a>

### Monthly mean of daily maximum temperature (NIST TN 1900 E2)

`nist-tn1900-e2-surface-temperature`

#### What is this project about?

TN 1900 E2: an observation equation t_i = τ + E_i for 22 daily readings; Type A evaluation with 21 degrees of freedom.


#### Step 1. What do we measure, and how is it calculated?

The result is **tau** in °C (Mean daily maximum temperature in May 2012). It is not measured directly: it is calculated from the input quantities with the measurement model

```
tau = t
```


#### Step 2. The input quantities and their uncertainties

The model has 1 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **t**: Daily maximum readings of the thermometer. Measured 22 times (a **Type A** evaluation, from statistics). The estimate is the mean 25.5909 °C; the readings scatter with a standard deviation s = 4.09 °C, and the standard uncertainty of the mean is u = s/√n = 4.09/√22 = 0.872 °C, with ν = n − 1 = 21 degrees of freedom.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 25.59 °C
- standard uncertainty u = 0.872 °C
- coverage factor k = 2.08

Reference: NIST TN 1900 E2: t̄ = 25.6 °C, u = 0.872 °C, k = 2.08, 95 % interval (23.8, 27.4) °C; 25.59 °C from the text.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Reporting with the CMC (Intermediate)

ILAC P14 says that a calibration certificate must not state an uncertainty smaller than the laboratory's calibration and measurement capability. The program applies this floor when it reports.

<a id="ilac-p14-cmc-floor-weight-10kg"></a>

### Reporting rule of ILAC P14 5.5: the CMC as the smallest reportable uncertainty (EA-4/02 S2 weight, illustrative CMC)

`ilac-p14-cmc-floor-weight-10kg`

#### What is this project about?

The 10 kg weight of EA-4/02 S2 (U = 58 mg) reported by a laboratory whose accredited CMC is taken, for illustration, as 80 mg: the reported U is the CMC, not the evaluated 58 mg.


#### Step 1. What do we measure, and how is it calculated?

The result is **m_X** in mg (Deviation of the weight from 10 kg). It is not measured directly: it is calculated from the input quantities with the measurement model

```
m_X = m_S + dm_D + dm + dm_C + dB
```


#### Step 2. The input quantities and their uncertainties

The model has 5 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **m_S**: Standard (certificate: +5 mg, U = 45 mg, k = 2). Best value 5 mg. The source (e.g. a certificate) gives the expanded uncertainty U = 45 mg with k = 2, so the standard uncertainty is u = U/k = 45/2 = 22.5 mg (normal distribution).
2. **dm_D**: Drift of the standard since its last calibration. Only limits are known: the value lies between -15 and 15 mg (0 ± 15), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 15/√3 = 8.66 mg.
3. **dm**: Observed difference (mean of 3, s_p = 25 mg). Best value 20 mg with standard uncertainty u = 14.434 mg (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **dm_C**: Comparator eccentricity and magnetic effects. Only limits are known: the value lies between -10 and 10 mg (0 ± 10), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 10/√3 = 5.77 mg.
5. **dB**: Air buoyancy. Only limits are known: the value lies between -10 and 10 mg (0 ± 10), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 10/√3 = 5.77 mg.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 25 mg
- standard uncertainty u = 29.2 mg
- expanded uncertainty U = 58 mg

Reference: EA-4/02 M:2022 S2 (U = 58 mg) and ILAC P14:09/2020 5.5; the CMC is illustrative.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## When one rectangular term dominates (Intermediate)

A voltmeter whose resolution dominates. UKAS M3003 gives a special coverage factor for this case, because the result is closer to rectangular than to normal.

<a id="m3003-c5-dvm-dominant-rectangular"></a>

### Digital voltmeter with a dominant resolution term (UKAS M3003 Appendix C)

`m3003-c5-dvm-dominant-rectangular`

#### What is this project about?

A DVM reading 1.001 V for an applied 1.000 00 V; the rounding of the 1 mV digit dominates, so k for 95.45 % comes from the convolution of a rectangular and a normal distribution.


#### Step 1. What do we measure, and how is it calculated?

The result is **dV** in mV (Error of the voltmeter at 1 V). It is not measured directly: it is calculated from the input quantities with the measurement model

```
dV = V_ind - V_ref + d_IT
```


#### Step 2. The input quantities and their uncertainties

The model has 3 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **V_ind**: Indication (1 mV digit). Only limits are known: the value lies between 1000.5 and 1001.5 mV (1001 ± 0.5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.5/√3 = 0.289 mV.
2. **V_ref**: Applied reference voltage (U = 0.19 mV, k = 2). Best value 1000 mV. The source (e.g. a certificate) gives the expanded uncertainty U = 0.19 mV with k = 2, so the standard uncertainty is u = U/k = 0.19/2 = 0.0950 mV (normal distribution).
3. **d_IT**: Drift and temperature effects. Only limits are known: the value lies between -0.05 and 0.05 mV (0 ± 0.05), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.05/√3 = 0.0289 mV.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It chooses the coverage factor k for a coverage probability of 95.5 %, using the effective degrees of freedom (Welch–Satterthwaite), and gives the **expanded uncertainty** U = k·u_c.

It also uses the **Monte Carlo method**: it draws 200 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 200 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 1 mV
- standard uncertainty u = 0.305 mV
- coverage factor k = 1.77
- expanded uncertainty U = 0.54 mV

Reference: UKAS M3003 ed. 6 C.8–C.10: u_c = 0.305 mV, k = 1.77, U = 0.54 mV.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## A multimeter specification across a range (Intermediate)

A 6½-digit multimeter on its 1 V range: some contributions are relative to the reading (ppm), others are absolute (µV). The examples show the uncertainty at several points of the range and the coverage factor when the specification dominates.

<a id="m3003-l3-dmm-1v-range-095v"></a>

### DMM on the 1 V range: budget at 0.95 V (UKAS M3003 L.3)

`m3003-l3-dmm-1v-range-095v`

#### What is this project about?

Error of a 6½-digit DMM on its 1 V range at 0.95 V, calibrated with a multifunction calibrator; relative (ppm) and absolute (µV) contributions combined at one reading.


#### Step 1. What do we measure, and how is it calculated?

The result is **dI** in µV (Error of the DMM indication). It is not measured directly: it is calculated from the input quantities with the measurement model

```
dI = I_DMM*1e-6*(r_cal + r_spec + r_rep) + a_cal + a_spec + a_T + a_CM + a_res
```


#### Step 2. The input quantities and their uncertainties

The model has 9 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **I_DMM**: Indication. An exact value, 950000 µV, with no uncertainty (for example a defined constant or a nominal value).
2. **r_cal**: Calibration of the calibrator, relative (U = 2.8 ppm, k = 2). Best value 0 ppm. The source (e.g. a certificate) gives the expanded uncertainty U = 2.8 ppm with k = 2, so the standard uncertainty is u = U/k = 2.8/2 = 1.40 ppm (normal distribution).
3. **r_spec**: Specification of the calibrator, relative (±8 ppm). Only limits are known: the value lies between -8 and 8 ppm (0 ± 8), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 8/√3 = 4.62 ppm.
4. **r_rep**: Repeatability (2.5 ppm, ν = 9, one reading). Mean of 1 readings, 0 ppm. Instead of the scatter of these few readings, a standard deviation s_p = 2.5 ppm determined earlier from more data (ν = 9) is used, so u = s_p/√n = 2.50 ppm.
5. **a_cal**: Calibration of the calibrator, absolute (U = 0.5 µV, k = 2). Best value 0 µV. The source (e.g. a certificate) gives the expanded uncertainty U = 0.5 µV with k = 2, so the standard uncertainty is u = U/k = 0.5/2 = 0.250 µV (normal distribution).
6. **a_spec**: Specification of the calibrator, absolute (±2 µV). Only limits are known: the value lies between -2 and 2 µV (0 ± 2), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 2/√3 = 1.15 µV.
7. **a_T**: Thermoelectric voltages. Only limits are known: the value lies between -1 and 1 µV (0 ± 1), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 1/√3 = 0.577 µV.
8. **a_CM**: Common-mode effects. Only limits are known: the value lies between -1 and 1 µV (0 ± 1), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 1/√3 = 0.577 µV.
9. **a_res**: Resolution of the DMM. Only limits are known: the value lies between -0.5 and 0.5 µV (0 ± 0.5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.5/√3 = 0.289 µV.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 200 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 200 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 0 µV
- standard uncertainty u = 5.37 µV
- expanded uncertainty U = 10.73 µV

Reference: UKAS M3003 ed. 6 L.3.19 (budget at 0.95 V): u_c = 5.37 µV, U = 10.73 µV.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="m3003-l3-dmm-1v-range-1v-dominant"></a>

### DMM on the 1 V range at full scale: coverage factor for a dominant specification term (UKAS M3003 L.3.20)

`m3003-l3-dmm-1v-range-1v-dominant`

#### What is this project about?

Error of a 6½-digit DMM on its 1 V range at 1.0 V, calibrated with a multifunction calibrator; relative (ppm) and absolute (µV) contributions combined at one reading.


#### Step 1. What do we measure, and how is it calculated?

The result is **dI** in µV (Error of the DMM indication). It is not measured directly: it is calculated from the input quantities with the measurement model

```
dI = I_DMM*1e-6*(r_cal + r_spec + r_rep) + a_cal + a_spec + a_T + a_CM + a_res
```


#### Step 2. The input quantities and their uncertainties

The model has 9 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **I_DMM**: Indication. An exact value, 1000000 µV, with no uncertainty (for example a defined constant or a nominal value).
2. **r_cal**: Calibration of the calibrator, relative (U = 2.8 ppm, k = 2). Best value 0 ppm. The source (e.g. a certificate) gives the expanded uncertainty U = 2.8 ppm with k = 2, so the standard uncertainty is u = U/k = 2.8/2 = 1.40 ppm (normal distribution).
3. **r_spec**: Specification of the calibrator, relative (±8 ppm). Only limits are known: the value lies between -8 and 8 ppm (0 ± 8), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 8/√3 = 4.62 ppm.
4. **r_rep**: Repeatability (2.5 ppm, ν = 9, one reading). Mean of 1 readings, 0 ppm. Instead of the scatter of these few readings, a standard deviation s_p = 2.5 ppm determined earlier from more data (ν = 9) is used, so u = s_p/√n = 2.50 ppm.
5. **a_cal**: Calibration of the calibrator, absolute (U = 0.5 µV, k = 2). Best value 0 µV. The source (e.g. a certificate) gives the expanded uncertainty U = 0.5 µV with k = 2, so the standard uncertainty is u = U/k = 0.5/2 = 0.250 µV (normal distribution).
6. **a_spec**: Specification of the calibrator, absolute (±2 µV). Only limits are known: the value lies between -2 and 2 µV (0 ± 2), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 2/√3 = 1.15 µV.
7. **a_T**: Thermoelectric voltages. Only limits are known: the value lies between -1 and 1 µV (0 ± 1), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 1/√3 = 0.577 µV.
8. **a_CM**: Common-mode effects. Only limits are known: the value lies between -1 and 1 µV (0 ± 1), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 1/√3 = 0.577 µV.
9. **a_res**: Resolution of the DMM. Only limits are known: the value lies between -0.5 and 0.5 µV (0 ± 0.5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.5/√3 = 0.289 µV.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It chooses the coverage factor k for a coverage probability of 95.5 %, using the effective degrees of freedom (Welch–Satterthwaite), and gives the **expanded uncertainty** U = k·u_c.

It also uses the **Monte Carlo method**: it draws 200 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 200 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 0 µV
- coverage factor k = 1.898

Reference: UKAS M3003 ed. 6 L.3.20: k = 1.898 for u_R/u_N = 1.435.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="m3003-l3-dmm-1v-range-500mv"></a>

### DMM on the 1 V range: uncertainty at 500 mV from the range formula (UKAS M3003 L.3)

`m3003-l3-dmm-1v-range-500mv`

#### What is this project about?

Error of a 6½-digit DMM on its 1 V range at 0.5 V, calibrated with a multifunction calibrator; relative (ppm) and absolute (µV) contributions combined at one reading.


#### Step 1. What do we measure, and how is it calculated?

The result is **dI** in µV (Error of the DMM indication). It is not measured directly: it is calculated from the input quantities with the measurement model

```
dI = I_DMM*1e-6*(r_cal + r_spec + r_rep) + a_cal + a_spec + a_T + a_CM + a_res
```


#### Step 2. The input quantities and their uncertainties

The model has 9 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **I_DMM**: Indication. An exact value, 500000 µV, with no uncertainty (for example a defined constant or a nominal value).
2. **r_cal**: Calibration of the calibrator, relative (U = 2.8 ppm, k = 2). Best value 0 ppm. The source (e.g. a certificate) gives the expanded uncertainty U = 2.8 ppm with k = 2, so the standard uncertainty is u = U/k = 2.8/2 = 1.40 ppm (normal distribution).
3. **r_spec**: Specification of the calibrator, relative (±8 ppm). Only limits are known: the value lies between -8 and 8 ppm (0 ± 8), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 8/√3 = 4.62 ppm.
4. **r_rep**: Repeatability (2.5 ppm, ν = 9, one reading). Mean of 1 readings, 0 ppm. Instead of the scatter of these few readings, a standard deviation s_p = 2.5 ppm determined earlier from more data (ν = 9) is used, so u = s_p/√n = 2.50 ppm.
5. **a_cal**: Calibration of the calibrator, absolute (U = 0.5 µV, k = 2). Best value 0 µV. The source (e.g. a certificate) gives the expanded uncertainty U = 0.5 µV with k = 2, so the standard uncertainty is u = U/k = 0.5/2 = 0.250 µV (normal distribution).
6. **a_spec**: Specification of the calibrator, absolute (±2 µV). Only limits are known: the value lies between -2 and 2 µV (0 ± 2), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 2/√3 = 1.15 µV.
7. **a_T**: Thermoelectric voltages. Only limits are known: the value lies between -1 and 1 µV (0 ± 1), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 1/√3 = 0.577 µV.
8. **a_CM**: Common-mode effects. Only limits are known: the value lies between -1 and 1 µV (0 ± 1), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 1/√3 = 0.577 µV.
9. **a_res**: Resolution of the DMM. Only limits are known: the value lies between -0.5 and 0.5 µV (0 ± 0.5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.5/√3 = 0.289 µV.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 200 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 200 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 0 µV
- expanded uncertainty U = 6.2 µV

Reference: UKAS M3003 ed. 6 L.3.14: U = 6.2 µV at 500 mV.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Standardising a sodium hydroxide solution (Intermediate)

Titration against a weighed primary standard. The example uses sub-models (the mass, the volume of the burette) and shows that repeatability can be put into one combined term.

<a id="quam-a2-naoh-standardisation-submodels"></a>

### NaOH standardisation with sub-models

`quam-a2-naoh-standardisation-submodels`

#### What is this project about?

QUAM Example A2 with mass, molar mass and volume built from their components.


#### Step 1. What do we measure, and how is it calculated?

The result is **c_NaOH** in mol/L (Concentration of the NaOH solution). It is not measured directly: it is calculated from the input quantities with the measurement model

```
c_NaOH = 1000 * m_KHP * P_KHP / (M_KHP * V_T) * rep
```

To keep the equation readable, 3 intermediate quantities are calculated first:

- `m_KHP = m_gross - m_tare`
- `M_KHP = 8*A_C + 5*A_H + 4*A_O + A_K`
- `V_T = 18.64 + dV_cal + dV_temp`


#### Step 2. The input quantities and their uncertainties

The model has 10 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **rep**: Repeatability factor. Best value 1 with standard uncertainty u = 0.0005 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **m_gross**: Container and KHP. Only limits are known: the value lies between 60.545 and 60.54515 g (60.545 ± 0.00015), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.00015/√3 = 8.66 × 10⁻⁵ g.
3. **m_tare**: Empty container. Only limits are known: the value lies between 60.15605 and 60.156 g (60.1562 ± 0.00015), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.00015/√3 = 8.66 × 10⁻⁵ g.
4. **P_KHP**: Purity of KHP (1.0000 ± 0.0005). Only limits are known: the value lies between 0.9995 and 1.0005 (1 ± 0.0005), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.0005/√3 = 0.000289.
5. **A_C**: Atomic weight of C. Only limits are known: the value lies between 12.0099 and 12.0115 g/mol (12.0107 ± 0.0008), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.0008/√3 = 0.000462 g/mol.
6. **A_H**: Atomic weight of H. Only limits are known: the value lies between 1.00787 and 1.00801 g/mol (1.00794 ± 0.00007), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.00007/√3 = 4.04 × 10⁻⁵ g/mol.
7. **A_O**: Atomic weight of O. Only limits are known: the value lies between 15.9991 and 16.000 g/mol (15.9994 ± 0.0003), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.0003/√3 = 0.000173 g/mol.
8. **A_K**: Atomic weight of K. Only limits are known: the value lies between 39.0982 and 39.098 g/mol (39.0983 ± 0.0001), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.0001/√3 = 5.77 × 10⁻⁵ g/mol.
9. **dV_cal**: Burette calibration. The value lies between -0.03 and 0.03 mL, and values near the centre 0 are more likely than values near the limits (triangular distribution). u = a/√6 = 0.03/√6 = 0.0122 mL.
10. **dV_temp**: Temperature effect (±3 °C at 95 %). Best value 0 mL with standard uncertainty u = 0.0061071 mL (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 0.102136 mol/L
- expanded uncertainty U = 0.0002 mol/L

Reference: QUAM:2012 A2.4–A2.6, result (0.1021 ± 0.0002) mol/L.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="quam-a2-naoh-standardisation"></a>

### Standardising a sodium hydroxide solution

`quam-a2-naoh-standardisation`

#### What is this project about?

QUAM Example A2: NaOH standardised against potassium hydrogen phthalate (KHP); summary budget with the guide's standard uncertainties.


#### Step 1. What do we measure, and how is it calculated?

The result is **c_NaOH** in mol/L (Concentration of the NaOH solution). It is not measured directly: it is calculated from the input quantities with the measurement model

```
c_NaOH = 1000 * m_KHP * P_KHP / (M_KHP * V_T) * rep
```


#### Step 2. The input quantities and their uncertainties

The model has 5 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **rep**: Repeatability factor. Best value 1 with standard uncertainty u = 0.0005 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **m_KHP**: Mass of KHP. Best value 0.3888 g with standard uncertainty u = 0.00013 g (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **P_KHP**: Purity of KHP. Best value 1 with standard uncertainty u = 0.00029 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **M_KHP**: Molar mass of KHP. Best value 204.2212 g/mol with standard uncertainty u = 0.0038 g/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
5. **V_T**: Titration volume of NaOH. Best value 18.64 mL with standard uncertainty u = 0.013 mL (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 0.102136 mol/L
- standard uncertainty u = 0.000099 mol/L

Reference: QUAM:2012 Table A2.3 (c = 0.102136 mol/L, u_c = 0.000099 mol/L).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Titration of hydrochloric acid (Intermediate)

The concentration of HCl from a titration with standardised NaOH. The uncertainty of the NaOH from the previous example is carried over: results of one evaluation become inputs of the next.

<a id="quam-a3-hcl-titration"></a>

### Acid–base titration of hydrochloric acid

`quam-a3-hcl-titration`

#### What is this project about?

QUAM Example A3: HCl titrated with NaOH that was itself standardised against KHP.


#### Step 1. What do we measure, and how is it calculated?

The result is **c_HCl** in mol/L (Concentration of HCl). It is not measured directly: it is calculated from the input quantities with the measurement model

```
c_HCl = 1000 * m_KHP * P_KHP * V_T2 / (V_T1 * M_KHP * V_HCl) * rep
```


#### Step 2. The input quantities and their uncertainties

The model has 7 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **rep**: Repeatability factor. Best value 1 with standard uncertainty u = 0.001 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **m_KHP**: Mass of KHP. Best value 0.3888 g with standard uncertainty u = 0.00012 g (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **P_KHP**: Purity of KHP. Best value 1 with standard uncertainty u = 0.00029 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **V_T2**: NaOH volume for the HCl titration. Best value 14.89 mL with standard uncertainty u = 0.014 mL (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
5. **V_T1**: NaOH volume for the standardisation. Best value 18.64 mL with standard uncertainty u = 0.015 mL (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
6. **M_KHP**: Molar mass of KHP. Best value 204.2212 g/mol with standard uncertainty u = 0.0038 g/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
7. **V_HCl**: Aliquot of HCl. Best value 15 mL with standard uncertainty u = 0.011 mL (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 0.101387 mol/L
- standard uncertainty u = 0.00018276 mol/L
- expanded uncertainty U = 0.0004 mol/L

Reference: QUAM:2012 Table A3.3 (u_c = √3.34 × 10⁻⁸ mol/L) and result (0.1014 ± 0.0004) mol/L.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Pesticides in bread (Intermediate)

Two ways to the same answer: a budget built from the steps of the method, and a top-down estimate from validation data (precision and recovery). Compare which components each approach captures.

<a id="quam-a4-pesticides-combination"></a>

### Organophosphorus pesticides in bread: combination

`quam-a4-pesticides-combination`

#### What is this project about?

QUAM Example A4: the relative uncertainty from in-house validation data (precision, recovery, homogeneity) combined in a multiplicative model at P = 1.


#### Step 1. What do we measure, and how is it calculated?

The result is **P_op** (Pesticide level relative to the measured value). It is not measured directly: it is calculated from the input quantities with the measurement model

```
P_op = F_I * F_hom / Rec
```


#### Step 2. The input quantities and their uncertainties

The model has 3 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **F_I**: Intermediate precision factor. Best value 1 with standard uncertainty u = 0.27 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **Rec**: Mean recovery. Best value 0.9 with standard uncertainty u = 0.043 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **F_hom**: Sample inhomogeneity factor. Best value 1 with standard uncertainty u = 0.2 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 1.1111
- standard uncertainty u = 0.377

Reference: QUAM:2012 Table A4.5 (P = 1.1111, u = 0.377).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="quam-a4-pesticides-topdown"></a>

### Organophosphorus pesticides in bread from validation data

`quam-a4-pesticides-topdown`

#### What is this project about?

EURACHEM QUAM Example A4 as a top-down evaluation: precision from 15 duplicates (raw data), recovery from 42 spikes, homogeneity.


#### Step 1. The idea: uncertainty from the laboratory's own data

This is a **top-down** evaluation of **P_op**. Instead of modelling every step of the method, it uses data that the laboratory already has: how much the results scatter when the method is repeated on different days (precision) and how far they are, on average, from the true value (bias or recovery). Together these cover most sources of uncertainty at once.

The uncertainty is evaluated at 1 concentration levels: 1.


#### Step 2. The data

**Precision** (random effects), 1 entry:

- **precision**: 15 duplicate pairs; the standard deviation follows from the differences within the pairs.

**Recovery**: 1 recovery test (reference materials or spiked samples). The mean recovery R and its uncertainty describe how much of the analyte the method finds; if R differs significantly from 100 %, a correction is considered.

**Further components**, added as they are given:

- homogeneity


#### Step 3. What the program calculates

It combines the components as u = √(u(precision)² + u(bias)² + …), at each level, and multiplies by the coverage factor (k = 2 for about 95 %) to get the expanded uncertainty U. The results show u and U, absolute and relative, at every level.


> **Try:** Compare the size of the precision and bias components in the results. If bias dominates, a better reference material or a correction helps more than repeating measurements.


#### Finally: what the source expects

Reference: QUAM:2012 Tables A4.3–A4.5 (0.27, 0.048, 0.2 → 0.34; u = 0.377 at P = 1.1111).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Cadmium released from ceramics (Intermediate)

An empirical method: the result is defined by the procedure (time, temperature, acid). Effects of these conditions are included as uncertainty terms.

<a id="quam-a5-cadmium-release"></a>

### Cadmium released from ceramic ware

`quam-a5-cadmium-release`

#### What is this project about?

QUAM Example A5: empirical method (BS 6748); amount of Cd leached per unit area.


#### Step 1. What do we measure, and how is it calculated?

The result is **r** in mg/dm² (Cd released per unit area). It is not measured directly: it is calculated from the input quantities with the measurement model

```
r = c0 * V_L / a_V * f_acid * f_time * f_temp
```


#### Step 2. The input quantities and their uncertainties

The model has 6 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **c0**: Cd concentration in the leach solution. Best value 0.26 mg/L with standard uncertainty u = 0.018 mg/L (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **V_L**: Volume of the leachate. Best value 0.332 L with standard uncertainty u = 0.0018 L (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **a_V**: Surface area of the vessel. Best value 5.73 dm² with standard uncertainty u = 0.19 dm² (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **f_acid**: Acid concentration factor. Best value 1 with standard uncertainty u = 0.0008 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
5. **f_time**: Leaching time factor. Best value 1 with standard uncertainty u = 0.001 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
6. **f_temp**: Temperature factor. Best value 1 with standard uncertainty u = 0.06 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 0.015065 mg/dm²
- standard uncertainty u = 0.001465 mg/dm²
- expanded uncertainty U = 0.003 mg/dm²

Reference: QUAM:2012 Table A5.4 (r = 0.015065 mg/dm², spreadsheet u = 0.001465) and result (0.015 ± 0.003) mg/dm².

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Crude fibre in animal feed (Intermediate)

Another empirical method, evaluated from the precision of an interlaboratory study: the reproducibility of the method is the main part of the uncertainty.

<a id="quam-a6-crude-fibre"></a>

### Crude fibre in animal feed (empirical method)

`quam-a6-crude-fibre`

#### What is this project about?

EURACHEM QUAM Example A6: reproducibility s_R from a collaborative trial at three levels plus a drying allowance.


#### Step 1. The idea: uncertainty from the laboratory's own data

This is a **top-down** evaluation of **w(crude fibre)** (% m/m). Instead of modelling every step of the method, it uses data that the laboratory already has: how much the results scatter when the method is repeated on different days (precision) and how far they are, on average, from the true value (bias or recovery). Together these cover most sources of uncertainty at once.

The uncertainty is evaluated at 3 concentration levels: 2.5; 5; 10 % m/m.


#### Step 2. The data

**Precision** (random effects), 3 entries:

- **s_R (2.5)**: a standard deviation of 0.29 (e.g. from a control chart: within-laboratory reproducibility).
- **s_R (5)**: a standard deviation of 0.4 (e.g. from a control chart: within-laboratory reproducibility).
- **s_R (10)**: a standard deviation of 0.6 (e.g. from a control chart: within-laboratory reproducibility).

**Further components**, added as they are given:

- drying


#### Step 3. What the program calculates

It combines the components as u = √(u(precision)² + u(bias)² + …), at each level, and multiplies by the coverage factor (k = 2 for about 95 %) to get the expanded uncertainty U. The results show u and U, absolute and relative, at every level.


> **Try:** Compare the size of the precision and bias components in the results. If bias dominates, a better reference material or a correction helps more than repeating measurements.


#### Finally: what the source expects

Reference: QUAM:2012 Tables A6.4–A6.5.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Top-down for environmental analyses (Intermediate)

BOD in wastewater and PCB in sediment, each evaluated twice: once with a CRM and once with proficiency tests for the bias. See how the choice of bias data changes the result.

<a id="nordtest-8-bod-crm"></a>

### BOD in wastewater: control chart and one CRM

`nordtest-8-bod-crm`

#### What is this project about?

Nordtest TR 537 §8.2 Example A with the raw control data of Appendix 7.


#### Step 1. The idea: uncertainty from the laboratory's own data

This is a **top-down** evaluation of **BOD** (mg/L). Instead of modelling every step of the method, it uses data that the laboratory already has: how much the results scatter when the method is repeated on different days (precision) and how far they are, on average, from the true value (bias or recovery). Together these cover most sources of uncertainty at once.


#### Step 2. The data

**Precision** (random effects), 1 entry:

- **u(Rw)**: series

**Bias** (systematic effects): 1 source, e.g. certified reference materials or proficiency tests. From the differences between the laboratory's results and the reference values, the program estimates u(bias), which includes the uncertainty of the reference values themselves.


#### Step 3. What the program calculates

It combines the components as u = √(u(precision)² + u(bias)² + …), at each level, and multiplies by the coverage factor (k = 2 for about 95 %) to get the expanded uncertainty U. The results show u and U, absolute and relative, at every level.


> **Try:** Compare the size of the precision and bias components in the results. If bias dominates, a better reference material or a correction helps more than repeating measurements.


#### Finally: what the source expects

Reference: Nordtest TR 537 §8.2 Example A (u(Rw) 2.6 %, u(bias) 4.5 %, u_c 5.2 %, U 10.4 → 11 %).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="nordtest-8-bod-pt"></a>

### BOD in wastewater: control chart and proficiency tests

`nordtest-8-bod-pt`

#### What is this project about?

Nordtest TR 537 §8.2 Example B: u(bias) from three proficiency tests (Table 6, raw data).


#### Step 1. The idea: uncertainty from the laboratory's own data

This is a **top-down** evaluation of **BOD** (mg/L). Instead of modelling every step of the method, it uses data that the laboratory already has: how much the results scatter when the method is repeated on different days (precision) and how far they are, on average, from the true value (bias or recovery). Together these cover most sources of uncertainty at once.


#### Step 2. The data

**Precision** (random effects), 1 entry:

- **u(Rw)**: series

**Bias** (systematic effects): 1 source, e.g. certified reference materials or proficiency tests. From the differences between the laboratory's results and the reference values, the program estimates u(bias), which includes the uncertainty of the reference values themselves.


#### Step 3. What the program calculates

It combines the components as u = √(u(precision)² + u(bias)² + …), at each level, and multiplies by the coverage factor (k = 2 for about 95 %) to get the expanded uncertainty U. The results show u and U, absolute and relative, at every level.


> **Try:** Compare the size of the precision and bias components in the results. If bias dominates, a better reference material or a correction helps more than repeating measurements.


#### Finally: what the source expects

Reference: Nordtest TR 537 §8.2 Example B (u(bias) 4.12 %, u_c 4.87 %, U 9.7 → 10 %); tolerances cover the rounded biases of Table 6.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="nordtest-8-pcb-crm"></a>

### PCB in sediment: control sample and one CRM

`nordtest-8-pcb-crm`

#### What is this project about?

Nordtest TR 537 §8.3 Example C.


#### Step 1. The idea: uncertainty from the laboratory's own data

This is a **top-down** evaluation of **w(ΣPCB₇)** (µg/kg). Instead of modelling every step of the method, it uses data that the laboratory already has: how much the results scatter when the method is repeated on different days (precision) and how far they are, on average, from the true value (bias or recovery). Together these cover most sources of uncertainty at once.


#### Step 2. The data

**Precision** (random effects), 1 entry:

- **u(Rw)**: a standard deviation of 8 % (e.g. from a control chart: within-laboratory reproducibility).

**Bias** (systematic effects): 1 source, e.g. certified reference materials or proficiency tests. From the differences between the laboratory's results and the reference values, the program estimates u(bias), which includes the uncertainty of the reference values themselves.


#### Step 3. What the program calculates

It combines the components as u = √(u(precision)² + u(bias)² + …), at each level, and multiplies by the coverage factor (k = 2 for about 95 %) to get the expanded uncertainty U. The results show u and U, absolute and relative, at every level.


> **Try:** Compare the size of the precision and bias components in the results. If bias dominates, a better reference material or a correction helps more than repeating measurements.


#### Finally: what the source expects

Reference: Nordtest TR 537 §8.3 Example C (u(bias) 7.22 %, u_c 10.8 %, U 21.6 → 22 %); computed from 144 and 152 rather than the rounded 5.3 % and 4.6 %.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="nordtest-8-pcb-pt"></a>

### PCB in sediment: control sample and proficiency tests

`nordtest-8-pcb-pt`

#### What is this project about?

Nordtest TR 537 §8.3 Example D.


#### Step 1. The idea: uncertainty from the laboratory's own data

This is a **top-down** evaluation of **w(ΣPCB₇)** (µg/kg). Instead of modelling every step of the method, it uses data that the laboratory already has: how much the results scatter when the method is repeated on different days (precision) and how far they are, on average, from the true value (bias or recovery). Together these cover most sources of uncertainty at once.


#### Step 2. The data

**Precision** (random effects), 1 entry:

- **u(Rw)**: a standard deviation of 8 % (e.g. from a control chart: within-laboratory reproducibility).

**Bias** (systematic effects): 1 source, e.g. certified reference materials or proficiency tests. From the differences between the laboratory's results and the reference values, the program estimates u(bias), which includes the uncertainty of the reference values themselves.


#### Step 3. What the program calculates

It combines the components as u = √(u(precision)² + u(bias)² + …), at each level, and multiplies by the coverage factor (k = 2 for about 95 %) to get the expanded uncertainty U. The results show u and U, absolute and relative, at every level.


> **Try:** Compare the size of the precision and bias components in the results. If bias dominates, a better reference material or a correction helps more than repeating measurements.


#### Finally: what the source expects

Reference: Nordtest TR 537 §8.3 Example D (RMS_bias 7.6 %, u(bias) 8.1 %, u_c 11.4 %, U 22.8 → 23 %).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Uncertainty from in-house precision and recovery (Intermediate)

The 2026 Eurachem/CITAC guide (MUPR) evaluates uncertainty from data every laboratory has: precision at several levels and recovery from reference materials or spiked samples. The examples cover pooled and concentration-dependent precision and different recovery designs.

<a id="mupr-b1-nitrate"></a>

### Nitrate in drinking water: two precision intervals

`mupr-b1-nitrate`

#### What is this project about?

MUPR Example B1: absolute precision below 0.4 mg/L, relative above; recovery from 20 spiked waters; stock solution and dilution components.


#### Step 1. The idea: uncertainty from the laboratory's own data

This is a **top-down** evaluation of **c(NO₃⁻)** (mg/L). Instead of modelling every step of the method, it uses data that the laboratory already has: how much the results scatter when the method is repeated on different days (precision) and how far they are, on average, from the true value (bias or recovery). Together these cover most sources of uncertainty at once.

The uncertainty is evaluated at 2 concentration levels: 0.261; 50 mg/L.


#### Step 2. The data

**Precision** (random effects), 2 entries:

- **s_I⟨I⟩**: a standard deviation of 0.0328 (e.g. from a control chart: within-laboratory reproducibility).
- **s′_I⟨II⟩**: a standard deviation of 8.43 % (e.g. from a control chart: within-laboratory reproducibility).

**Recovery**: 1 recovery test (reference materials or spiked samples). The mean recovery R and its uncertainty describe how much of the analyte the method finds; if R differs significantly from 100 %, a correction is considered.

**Further components**, added as they are given:

- u′(DSS)
- u′(F)


#### Step 3. What the program calculates

It combines the components as u = √(u(precision)² + u(bias)² + …), at each level, and multiplies by the coverage factor (k = 2 for about 95 %) to get the expanded uncertainty U. The results show u and U, absolute and relative, at every level.


> **Try:** Compare the size of the precision and bias components in the results. If bias dominates, a better reference material or a correction helps more than repeating measurements.


#### Finally: what the source expects

Reference: MUPR Example B1, Eq. (B1.3) and B1.7.iv (U = 0.0676 mg/L at 0.261 mg/L; 9.26 mg/L at 50 mg/L, computed by the guide with u′(R̄) rounded to 0.0285; unrounded 9.27 mg/L).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="mupr-b2-pooled-precision"></a>

### Nitrate: pooled precision and two recovery levels

`mupr-b2-pooled-precision`

#### What is this project about?

MUPR Example B2: precision pooled from several materials (raw data for two of them), transition concentration and overall recovery.


#### Step 1. The idea: uncertainty from the laboratory's own data

This is a **top-down** evaluation of **c(NO₃⁻)** (mg/L). Instead of modelling every step of the method, it uses data that the laboratory already has: how much the results scatter when the method is repeated on different days (precision) and how far they are, on average, from the true value (bias or recovery). Together these cover most sources of uncertainty at once.

The uncertainty is evaluated at 1 concentration levels: 50 mg/L.


#### Step 2. The data

**Precision** (random effects), 2 entries:

- **s_I(P)⟨I⟩**: 3 groups of repeated results (e.g. days); an analysis of variance separates repeatability from between-day variation.
- **s′_I(P)⟨II⟩**: 2 groups of repeated results (e.g. days); an analysis of variance separates repeatability from between-day variation.

**Recovery**: 2 recovery tests (reference materials or spiked samples). The mean recovery R and its uncertainty describe how much of the analyte the method finds; if R differs significantly from 100 %, a correction is considered.

**Further components**, added as they are given:

- u′(DSS)
- u′(F)


#### Step 3. What the program calculates

It combines the components as u = √(u(precision)² + u(bias)² + …), at each level, and multiplies by the coverage factor (k = 2 for about 95 %) to get the expanded uncertainty U. The results show u and U, absolute and relative, at every level.


> **Try:** Compare the size of the precision and bias components in the results. If bias dominates, a better reference material or a correction helps more than repeating measurements.


#### Finally: what the source expects

Reference: MUPR Example B2, Eq. (B2.1)–(B2.7).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="mupr-b2-replicates-on-two-days"></a>

### Nitrate: result as the mean of 3 replicates on 2 days

`mupr-b2-replicates-on-two-days`

#### What is this project about?

MUPR Example B2, Eq. (B2.9): precision of a mean of n results on each of p days (Eq. 5).


#### Step 1. The idea: uncertainty from the laboratory's own data

This is a **top-down** evaluation of **c(NO₃⁻)** (mg/L). Instead of modelling every step of the method, it uses data that the laboratory already has: how much the results scatter when the method is repeated on different days (precision) and how far they are, on average, from the true value (bias or recovery). Together these cover most sources of uncertainty at once.

The uncertainty is evaluated at 1 concentration levels: 57.5 mg/L.


#### Step 2. The data

**Precision** (random effects), 1 entry:

- **s′_I(P)⟨II⟩**: 2 groups of repeated results (e.g. days); an analysis of variance separates repeatability from between-day variation.

**Recovery**: 2 recovery tests (reference materials or spiked samples). The mean recovery R and its uncertainty describe how much of the analyte the method finds; if R differs significantly from 100 %, a correction is considered.

**Further components**, added as they are given:

- u′(DSS)
- u′(F)


#### Step 3. What the program calculates

It combines the components as u = √(u(precision)² + u(bias)² + …), at each level, and multiplies by the coverage factor (k = 2 for about 95 %) to get the expanded uncertainty U. The results show u and U, absolute and relative, at every level.


> **Try:** Compare the size of the precision and bias components in the results. If bias dominates, a better reference material or a correction helps more than repeating measurements.


#### Finally: what the source expects

Reference: MUPR Example B2, Eq. (B2.9) (U = 7.06 mg/L).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="mupr-b3-linear-precision"></a>

### Nitrate: precision as a linear function of concentration

`mupr-b3-linear-precision`

#### What is this project about?

MUPR Example B3: the intermediate precision of four materials is modelled as s_I = a + b·c by least squares and applied to a diluted sample.


#### Step 1. The idea: uncertainty from the laboratory's own data

This is a **top-down** evaluation of **c(NO₃⁻)** (mg/L). Instead of modelling every step of the method, it uses data that the laboratory already has: how much the results scatter when the method is repeated on different days (precision) and how far they are, on average, from the true value (bias or recovery). Together these cover most sources of uncertainty at once.

The uncertainty is evaluated at 1 concentration levels: 57.5 mg/L.


#### Step 2. The data

**Precision** (random effects), 1 entry:

- **s_I(c)**: linear

**Recovery**: 2 recovery tests (reference materials or spiked samples). The mean recovery R and its uncertainty describe how much of the analyte the method finds; if R differs significantly from 100 %, a correction is considered.

**Further components**, added as they are given:

- u′(DSS)
- u′(F)


#### Step 3. What the program calculates

It combines the components as u = √(u(precision)² + u(bias)² + …), at each level, and multiplies by the coverage factor (k = 2 for about 95 %) to get the expanded uncertainty U. The results show u and U, absolute and relative, at every level.


> **Try:** Compare the size of the precision and bias components in the results. If bias dominates, a better reference material or a correction helps more than repeating measurements.


#### Finally: what the source expects

Reference: MUPR Example B3, Eq. (B3.1) (U = 7.28 mg/L with the printed intercept 0.0233; 7.27 mg/L with the least-squares 0.02324).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="mupr-b4-arsenic-one-crm"></a>

### Arsenic in sediment: one CRM

`mupr-b4-arsenic-one-crm`

#### What is this project about?

MUPR Example B4: pooled relative precision and the recovery of one CRM.


#### Step 1. The idea: uncertainty from the laboratory's own data

This is a **top-down** evaluation of **w(As)** (mg/kg). Instead of modelling every step of the method, it uses data that the laboratory already has: how much the results scatter when the method is repeated on different days (precision) and how far they are, on average, from the true value (bias or recovery). Together these cover most sources of uncertainty at once.

The uncertainty is evaluated at 1 concentration levels: 16 mg/kg.


#### Step 2. The data

**Precision** (random effects), 2 entries:

- **s_I⟨I⟩**: 3 groups of repeated results (e.g. days); an analysis of variance separates repeatability from between-day variation.
- **s′_I⟨II⟩**: 3 groups of repeated results (e.g. days); an analysis of variance separates repeatability from between-day variation.

**Recovery**: 1 recovery test (reference materials or spiked samples). The mean recovery R and its uncertainty describe how much of the analyte the method finds; if R differs significantly from 100 %, a correction is considered.


#### Step 3. What the program calculates

It combines the components as u = √(u(precision)² + u(bias)² + …), at each level, and multiplies by the coverage factor (k = 2 for about 95 %) to get the expanded uncertainty U. The results show u and U, absolute and relative, at every level.


> **Try:** Compare the size of the precision and bias components in the results. If bias dominates, a better reference material or a correction helps more than repeating measurements.


#### Finally: what the source expects

Reference: MUPR Example B4, Eq. (B4.1)–(B4.5).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="mupr-b5-crm-and-spiked-samples"></a>

### Arsenic: CRM and spiked samples with matrix-dependent recovery

`mupr-b5-crm-and-spiked-samples`

#### What is this project about?

MUPR Example B5: recoveries vary with the matrix, so their standard deviation is included and k = t(2).


#### Step 1. The idea: uncertainty from the laboratory's own data

This is a **top-down** evaluation of **w(As)** (mg/kg). Instead of modelling every step of the method, it uses data that the laboratory already has: how much the results scatter when the method is repeated on different days (precision) and how far they are, on average, from the true value (bias or recovery). Together these cover most sources of uncertainty at once.

The uncertainty is evaluated at 1 concentration levels: 16 mg/kg.


#### Step 2. The data

**Precision** (random effects), 1 entry:

- **s′_I⟨II⟩**: 3 groups of repeated results (e.g. days); an analysis of variance separates repeatability from between-day variation.

**Recovery**: 3 recovery tests (reference materials or spiked samples). The mean recovery R and its uncertainty describe how much of the analyte the method finds; if R differs significantly from 100 %, a correction is considered.


#### Step 3. What the program calculates

It combines the components as u = √(u(precision)² + u(bias)² + …), at each level, and multiplies by the coverage factor (k = 2 for about 95 %) to get the expanded uncertainty U. The results show u and U, absolute and relative, at every level.


> **Try:** Compare the size of the precision and bias components in the results. If bias dominates, a better reference material or a correction helps more than repeating measurements.


#### Finally: what the source expects

Reference: MUPR Example B5, Eq. (B5.1)–(B5.6) (U = 0.395 w with t rounded to 4.30; with t₀.₉₇₅(2) = 4.303, 0.3955 w).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="mupr-b6-sediment-22-pt"></a>

### Trace element in sediment: heterogeneity and 22 proficiency tests

`mupr-b6-sediment-22-pt`

#### What is this project about?

MUPR Example B6: precision from a homogeneous material plus the heterogeneity of real samples (duplicate ranges), recovery from 22 PT results (raw data).


#### Step 1. The idea: uncertainty from the laboratory's own data

This is a **top-down** evaluation of **w** (mg/kg). Instead of modelling every step of the method, it uses data that the laboratory already has: how much the results scatter when the method is repeated on different days (precision) and how far they are, on average, from the true value (bias or recovery). Together these cover most sources of uncertainty at once.

The uncertainty is evaluated at 2 concentration levels: 5; 20 mg/kg.


#### Step 2. The data

**Precision** (random effects), 4 entries:

- **s_I(DS)⟨I⟩**: series
- **s_r(h)⟨I⟩**: 30 duplicate pairs; the standard deviation follows from the differences within the pairs.
- **s′_I(DS)⟨II⟩**: series
- **s′_r(h)⟨II⟩**: 30 duplicate pairs; the standard deviation follows from the differences within the pairs.

**Recovery**: 22 recovery tests (reference materials or spiked samples). The mean recovery R and its uncertainty describe how much of the analyte the method finds; if R differs significantly from 100 %, a correction is considered.


#### Step 3. What the program calculates

It combines the components as u = √(u(precision)² + u(bias)² + …), at each level, and multiplies by the coverage factor (k = 2 for about 95 %) to get the expanded uncertainty U. The results show u and U, absolute and relative, at every level.


> **Try:** Compare the size of the precision and bias components in the results. If bias dominates, a better reference material or a correction helps more than repeating measurements.


#### Finally: what the source expects

Reference: MUPR Example B6, Tables B6.1–B6.3 and Eq. (B6.1)–(B6.9); χ² recomputed with the weighted mean of Eq. (16).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="mupr-ex2-reference-materials"></a>

### Recovery from two CRMs and a proficiency-test sample

`mupr-ex2-reference-materials`

#### What is this project about?

MUPR Example 2: ortho-phosphate; mean recovery and its uncertainty from three reference materials.


#### Step 1. The idea: uncertainty from the laboratory's own data

This is a **top-down** evaluation of **c(PO₄-P)** (mg/L). Instead of modelling every step of the method, it uses data that the laboratory already has: how much the results scatter when the method is repeated on different days (precision) and how far they are, on average, from the true value (bias or recovery). Together these cover most sources of uncertainty at once.


#### Step 2. The data

**Recovery**: 3 recovery tests (reference materials or spiked samples). The mean recovery R and its uncertainty describe how much of the analyte the method finds; if R differs significantly from 100 %, a correction is considered.


#### Step 3. What the program calculates

It combines the components as u = √(u(precision)² + u(bias)² + …), at each level, and multiplies by the coverage factor (k = 2 for about 95 %) to get the expanded uncertainty U. The results show u and U, absolute and relative, at every level.


> **Try:** Compare the size of the precision and bias components in the results. If bias dominates, a better reference material or a correction helps more than repeating measurements.


#### Finally: what the source expects

Reference: MUPR Example 2 (R̄ = 1.008, u(R̄) = 0.03705).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="mupr-ex3-spiked-samples"></a>

### Recovery from spiked river waters

`mupr-ex3-spiked-samples`

#### What is this project about?

MUPR Example 3: two spiked samples with native analyte (Eq. 10–11).


#### Step 1. The idea: uncertainty from the laboratory's own data

This is a **top-down** evaluation of **c** (µg/L). Instead of modelling every step of the method, it uses data that the laboratory already has: how much the results scatter when the method is repeated on different days (precision) and how far they are, on average, from the true value (bias or recovery). Together these cover most sources of uncertainty at once.


#### Step 2. The data

**Recovery**: 2 recovery tests (reference materials or spiked samples). The mean recovery R and its uncertainty describe how much of the analyte the method finds; if R differs significantly from 100 %, a correction is considered.


#### Step 3. What the program calculates

It combines the components as u = √(u(precision)² + u(bias)² + …), at each level, and multiplies by the coverage factor (k = 2 for about 95 %) to get the expanded uncertainty U. The results show u and U, absolute and relative, at every level.


> **Try:** Compare the size of the precision and bias components in the results. If bias dominates, a better reference material or a correction helps more than repeating measurements.


#### Finally: what the source expects

Reference: MUPR Example 3 (R̄ = 96.2 %, u(R̄) = 10.02 %).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Control charts in practice (Intermediate)

The worked examples of the Nordtest handbook: X-charts, R-charts and r%-charts for duplicates, target limits from a required uncertainty, blanks, CRMs and the yearly review of the limits.

<a id="tr569-ex1-nickel-steel-xrf"></a>

### Nickel in low-alloy steel by XRF: target limits from the required uncertainty (Nordtest TR 569 Example 1)

`tr569-ex1-nickel-steel-xrf`

#### What is this project about?

X-chart of a steel test sample, mean over one year 4.58 weight-% with s = 0.026 weight-%; required expanded uncertainty U = 4 % (relative).


#### Step 1. The idea: watching a method over time

A **control chart** (Nordtest TR 569) plots the result of a control sample, measured with every series, against time. Warning limits (±2 s) and action limits (±3 s) around a central line show when the method is no longer in statistical control.

Limits: target limits, from the precision the method must reach.


> **Try:** In the results, look for points beyond the action limits and for two of three successive points beyond the warning limits: these are the rules that say "stop and find the cause".


#### Finally: what the source expects

Reference: TR 569 Example 1.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="tr569-ex10-boron-toys-target"></a>

### Boron migrating from toys by ICP-OES: target limits from the reproducibility of EN 71-3 (Nordtest TR 569 Example 10)

`tr569-ex10-boron-toys-target`

#### What is this project about?

No material for the whole procedure: an in-house synthetic standard of 5.0 µg/L monitors drift; s_target = 10 %, two thirds of the 15 % reproducibility of EN 71-3.


#### Step 1. The idea: watching a method over time

A **control chart** (Nordtest TR 569) plots the result of a control sample, measured with every series, against time. Warning limits (±2 s) and action limits (±3 s) around a central line show when the method is no longer in statistical control.

Limits: target limits, from the precision the method must reach.


> **Try:** In the results, look for points beyond the action limits and for two of three successive points beyond the warning limits: these are the rules that say "stop and find the cause".


#### Finally: what the source expects

Reference: TR 569 Example 10.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="tr569-ex12-pooling-sr-srw"></a>

### Repeatability and within-laboratory reproducibility from replicate control results (Nordtest TR 569 Example 12)

`tr569-ex12-pooling-sr-srw`

#### What is this project about?

Three replicates of a QC sample on each of eight days. The standard deviation of all 24 results together is too low an estimate of s_Rw and would give control limits that are too narrow.


#### Step 1. The idea: watching a method over time

A **control chart** (Nordtest TR 569) plots the result of a control sample, measured with every series, against time. Warning limits (±2 s) and action limits (±3 s) around a central line show when the method is no longer in statistical control.

8 control values are entered.


> **Try:** In the results, look for points beyond the action limits and for two of three successive points beyond the warning limits: these are the rules that say "stop and find the cause".


#### Finally: what the source expects

Reference: TR 569 Example 12; footnote 24 prints √(0.01541² + 0.02246²) = 0.0272, ten times too small (research log F53).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="tr569-ex2-cobalt-steel-xrf"></a>

### Cobalt in low-alloy steel by XRF: target limits from the limit of quantification (Nordtest TR 569 Example 2)

`tr569-ex2-cobalt-steel-xrf`

#### What is this project about?

X-chart at low concentration, mean 0.0768 weight-% with s = 0.00063 weight-%; the required LOQ of 0.01 weight-% is 6 to 10 times s, so s_target = 0.001 weight-%.


#### Step 1. The idea: watching a method over time

A **control chart** (Nordtest TR 569) plots the result of a control sample, measured with every series, against time. Warning limits (±2 s) and action limits (±3 s) around a central line show when the method is no longer in statistical control.

Limits: target limits, from the precision the method must reach.


> **Try:** In the results, look for points beyond the action limits and for two of three successive points beyond the warning limits: these are the rules that say "stop and find the cause".


#### Finally: what the source expects

Reference: TR 569 Example 2.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="tr569-ex3-ammonium-r"></a>

### Ammonium nitrogen in water: R-chart of duplicate test samples (Nordtest TR 569 Example 3)

`tr569-ex3-ammonium-r`

#### What is this project about?

One test sample below 100 µg/L analysed in duplicate in each run; the range of the duplicates is plotted.


#### Step 1. The idea: watching a method over time

A **control chart** (Nordtest TR 569) plots the result of a control sample, measured with every series, against time. Warning limits (±2 s) and action limits (±3 s) around a central line show when the method is no longer in statistical control.


> **Try:** In the results, look for points beyond the action limits and for two of three successive points beyond the warning limits: these are the rules that say "stop and find the cause".


#### Finally: what the source expects

Reference: TR 569 Example 3. The example calls s the pooled repeatability standard deviation; its value is the mean range divided by 1.128.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="tr569-ex3-ammonium-x"></a>

### Ammonium nitrogen in water: X-chart of a standard solution (Nordtest TR 569 Example 3)

`tr569-ex3-ammonium-x`

#### What is this project about?

Indophenol blue method, 20 µg/L control solution prepared from (NH₄)₂SO₄, independent of the NH₄Cl calibration standards; the mean of duplicates is plotted.


#### Step 1. The idea: watching a method over time

A **control chart** (Nordtest TR 569) plots the result of a control sample, measured with every series, against time. Warning limits (±2 s) and action limits (±3 s) around a central line show when the method is no longer in statistical control.

Limits: statistical, from the scatter of the control values themselves.


> **Try:** In the results, look for points beyond the action limits and for two of three successive points beyond the warning limits: these are the rules that say "stop and find the cause".


#### Finally: what the source expects

Reference: TR 569 Example 3.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="tr569-ex4-lead-water-icpms"></a>

### Lead in water by ICP-MS: X-chart of an in-house lake water (Nordtest TR 569 Example 4)

`tr569-ex4-lead-water-icpms`

#### What is this project about?

Low lead concentration (0.29 µg/L) in a preserved lake water, analysed once in each run; statistical limits.


#### Step 1. The idea: watching a method over time

A **control chart** (Nordtest TR 569) plots the result of a control sample, measured with every series, against time. Warning limits (±2 s) and action limits (±3 s) around a central line show when the method is no longer in statistical control.

Limits: statistical, from the scatter of the control values themselves.


> **Try:** In the results, look for points beyond the action limits and for two of three successive points beyond the warning limits: these are the rules that say "stop and find the cause".


#### Finally: what the source expects

Reference: TR 569 Example 4.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="tr569-ex5-arsenic-crm-tolerance"></a>

### Arsenic in a fish CRM by ICP-MS: chart with action limits only at ± 15 % (Nordtest TR 569 Example 5)

`tr569-ex5-arsenic-crm-tolerance`

#### What is this project about?

CRM NRC DORM-2 (dogfish muscle), 18.0 µg/g arsenic, analysed once in each run; the laboratory accepts deviations of up to 15 % from the reference value.


#### Step 1. The idea: watching a method over time

A **control chart** (Nordtest TR 569) plots the result of a control sample, measured with every series, against time. Warning limits (±2 s) and action limits (±3 s) around a central line show when the method is no longer in statistical control.

Limits: tolerance.


> **Try:** In the results, look for points beyond the action limits and for two of three successive points beyond the warning limits: these are the rules that say "stop and find the cause".


#### Finally: what the source expects

Reference: TR 569 Example 5.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="tr569-ex6-phosphorus-relative-range"></a>

### Total phosphorus in water: r%-chart of duplicates (Nordtest TR 569 Example 6)

`tr569-ex6-phosphorus-relative-range`

#### What is this project about?

Test samples above 10 µg/L analysed in duplicate; the absolute difference as per cent of the mean is plotted, since the relative standard deviation is about constant well above the LOQ.


#### Step 1. The idea: watching a method over time

A **control chart** (Nordtest TR 569) plots the result of a control sample, measured with every series, against time. Warning limits (±2 s) and action limits (±3 s) around a central line show when the method is no longer in statistical control.

Limits: statistical, from the scatter of the control values themselves.


> **Try:** In the results, look for points beyond the action limits and for two of three successive points beyond the warning limits: these are the rules that say "stop and find the cause".


#### Finally: what the source expects

Reference: TR 569 Example 6. The example rounds CV = 1.88/1.128 = 1.667 % to 1.67 % before multiplying by the factors; with the unrounded value the limits would be 4.72 % and 6.14 %.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="tr569-ex7-beta-hch-crm-target"></a>

### β-HCH in cod liver oil by GC-ECD: target limits around the reference value (Nordtest TR 569 Example 7)

`tr569-ex7-beta-hch-crm-target`

#### What is this project about?

CRM BCR 598, 16 µg/kg β-hexachlorocyclohexane, analysed once in each run; target s of 15 %.


#### Step 1. The idea: watching a method over time

A **control chart** (Nordtest TR 569) plots the result of a control sample, measured with every series, against time. Warning limits (±2 s) and action limits (±3 s) around a central line show when the method is no longer in statistical control.

Limits: target limits, from the precision the method must reach.


> **Try:** In the results, look for points beyond the action limits and for two of three successive points beyond the warning limits: these are the rules that say "stop and find the cause".


#### Finally: what the source expects

Reference: TR 569 Example 7.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="tr569-ex8-copper-r-review"></a>

### Copper in water by ICP-OES: R-chart of duplicates and its review (Nordtest TR 569 Example 8)

`tr569-ex8-copper-r-review`

#### What is this project about?

Duplicates of test samples; preliminary mean range 0.11 mg/L, s_r = 0.0975 mg/L; s_r of the last 60 runs 0.0957 mg/L.


#### Step 1. The idea: watching a method over time

A **control chart** (Nordtest TR 569) plots the result of a control sample, measured with every series, against time. Warning limits (±2 s) and action limits (±3 s) around a central line show when the method is no longer in statistical control.


> **Try:** In the results, look for points beyond the action limits and for two of three successive points beyond the warning limits: these are the rules that say "stop and find the cause".


#### Finally: what the source expects

Reference: TR 569 Example 8. The example prints F = 1.037; 0.0975²/0.0957² = 1.0380.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="tr569-ex8-copper-x-new-limits"></a>

### Copper in water by ICP-OES: new X-chart limits from all data (Nordtest TR 569 Example 8)

`tr569-ex8-copper-x-new-limits`

#### What is this project about?

After the review the limits are recalculated from the longer period: x̄ = 1.048 mg/L and s = 0.0822 mg/L.


#### Step 1. The idea: watching a method over time

A **control chart** (Nordtest TR 569) plots the result of a control sample, measured with every series, against time. Warning limits (±2 s) and action limits (±3 s) around a central line show when the method is no longer in statistical control.

Limits: statistical, from the scatter of the control values themselves.


> **Try:** In the results, look for points beyond the action limits and for two of three successive points beyond the warning limits: these are the rules that say "stop and find the cause".


#### Finally: what the source expects

Reference: TR 569 Example 8.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="tr569-ex8-copper-x-review"></a>

### Copper in water by ICP-OES: review of preliminary X-chart limits (Nordtest TR 569 Example 8)

`tr569-ex8-copper-x-review`

#### What is this project about?

Synthetic standard (1.00 ± 0.02 mg/L) measured in duplicate; preliminary limits from the first 60 runs, x̄ = 1.055 mg/L, s = 0.0667 mg/L. The last 60 values are reviewed: 11 outside the warning limits; one value more than 4 s from the central line is excluded, leaving 59 values with x̄ = 1.041 mg/L and s_Rw = 0.0834 mg/L.


#### Step 1. The idea: watching a method over time

A **control chart** (Nordtest TR 569) plots the result of a control sample, measured with every series, against time. Warning limits (±2 s) and action limits (±3 s) around a central line show when the method is no longer in statistical control.

Limits: statistical, from the scatter of the control values themselves.


> **Try:** In the results, look for points beyond the action limits and for two of three successive points beyond the warning limits: these are the rules that say "stop and find the cause".


#### Finally: what the source expects

Reference: TR 569 Example 8. The critical F of 1.67 is read from Table 3 at 60 and 60 degrees of freedom; the program computes F₀.₉₇₅(58, 59) exactly.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="tr569-ex9-zinc-procedural-blank"></a>

### Zinc in hydrogen peroxide: X-chart of a procedural blank (Nordtest TR 569 Example 9)

`tr569-ex9-zinc-procedural-blank`

#### What is this project about?

Ultrapure water taken through the whole procedure (evaporation of 50 mL, HCl, dilution to 5 mL, ICP-OES) to detect contamination; negative values are plotted as well.


#### Step 1. The idea: watching a method over time

A **control chart** (Nordtest TR 569) plots the result of a control sample, measured with every series, against time. Warning limits (±2 s) and action limits (±3 s) around a central line show when the method is no longer in statistical control.

Limits: statistical, from the scatter of the control values themselves.


> **Try:** In the results, look for points beyond the action limits and for two of three successive points beyond the warning limits: these are the rules that say "stop and find the cause".


#### Finally: what the source expects

Reference: TR 569 Example 9.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="tr569-ex11-micropipette-tolerance"></a>

### Bias of a 1 mL micropipette: chart with action limits at the tolerance (Nordtest TR 569 Example 11)

`tr569-ex11-micropipette-tolerance`

#### What is this project about?

A fixed 1 mL micropipette tested ten times (Nordtest TR 626); the mean volume is plotted against the nominal value with action limits at the laboratory's tolerance of ± 1 %.


#### Step 1. The idea: watching a method over time

A **control chart** (Nordtest TR 569) plots the result of a control sample, measured with every series, against time. Warning limits (±2 s) and action limits (±3 s) around a central line show when the method is no longer in statistical control.

Limits: tolerance.


> **Try:** In the results, look for points beyond the action limits and for two of three successive points beyond the warning limits: these are the rules that say "stop and find the cause".


#### Finally: what the source expects

Reference: TR 569 Example 11.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Decision limits (Intermediate)

When must a speeding ticket be issued, or a food sample be declared non-compliant? A decision limit is placed beyond the legal limit, so that a false accusation is very unlikely (guarded rejection).

<a id="jcgm106-8.3-nandrolone"></a>

### Nandrolone in animal products: decision limit for a screening test (JCGM 106 8.3.3)

`jcgm106-8.3-nandrolone`

#### What is this project about?

JCGM 106:2012 8.3.3 Example 2: threshold 2.00 µg/L; a result is suspect if the probability that the threshold is exceeded is at least 95 %; reproducibility s = 0.20 µg/L from ten spiked blanks, t distribution with ν = 9.


#### Step 1. The idea: does the item meet the specification?

The measured quantity c must be at most 2 µg/L. Because every measured value is uncertain, a result close to a limit could be on either side of it. The program calculates the **probability of conformity** and applies a **decision rule** (JCGM 106).

Decision rule: guarded rejection: an item is rejected only if it is clearly outside the limit (e.g. in law enforcement or anti-doping, where a false accusation must be avoided).


#### Finally: what the source expects

Reference: JCGM 106:2012 8.3.3, Example 2 (A = 2.37 µg/L).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="jcgm106-8.3-speed-limit"></a>

### Speed limit enforcement: guarded rejection (JCGM 106 8.3.3)

`jcgm106-8.3-speed-limit`

#### What is this project about?

JCGM 106:2012 8.3.3 Example 1: speed limit 100 km/h, radar with relative standard uncertainty 2 %; a ticket only if the probability that the limit was exceeded is at least 99.9 %.


#### Step 1. The idea: does the item meet the specification?

The measured quantity v must be at most 100 km/h. Because every measured value is uncertain, a result close to a limit could be on either side of it. The program calculates the **probability of conformity** and applies a **decision rule** (JCGM 106).

Decision rule: guarded rejection: an item is rejected only if it is clearly outside the limit (e.g. in law enforcement or anti-doping, where a false accusation must be avoided).


#### Finally: what the source expects

Reference: JCGM 106:2012 8.3.3, Example 1 (v_max ≈ 107 km/h).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Compliance with a specification (Intermediate)

The Eurachem/CITAC guide on compliance: guarded acceptance and rejection, a t distribution with few degrees of freedom, and a lognormal distribution for a large relative uncertainty near a limit.

<a id="eurachem-compliance-b1-nickel-steel"></a>

### Nickel in stainless steel: guarded acceptance with a two-sided specification (Eurachem/CITAC compliance guide, Example B1)

`eurachem-compliance-b1-nickel-steel`

#### What is this project about?

Specification 16.0–18.0 % Ni, U = 0.2 % (k = 2); acceptance zone where conformity is at least 95 % probable; measured 16.1 % Ni.


#### Step 1. The idea: does the item meet the specification?

The measured quantity w(Ni) must be between 16 and 18 %. Because every measured value is uncertain, a result close to a limit could be on either side of it. The program calculates the **probability of conformity** and applies a **decision rule** (JCGM 106).

Decision rule: guarded acceptance: the acceptance zone is made smaller than the tolerance by a guard band, so that accepting a bad item is unlikely (protects the customer).


#### Finally: what the source expects

Reference: Eurachem/CITAC 2021, Example B1: acceptance zone 16.2–18.0 → 16.2 % to 17.8 % Ni after rounding; 16.1 % Ni non-compliant.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="eurachem-compliance-b2-upper-limit-t"></a>

### Analyte below an upper limit, t distribution with ν = 8: guarded rejection (Eurachem/CITAC compliance guide, Example B2)

`eurachem-compliance-b2-upper-limit-t`

#### What is this project about?

Upper limit 200 ng/g, u = 2.2 ng/g with 8 degrees of freedom; non-compliant only if the probability above the limit exceeds 95 %; measured 203.7 ng/g.


#### Step 1. The idea: does the item meet the specification?

The measured quantity w must be at most 200 ng/g. Because every measured value is uncertain, a result close to a limit could be on either side of it. The program calculates the **probability of conformity** and applies a **decision rule** (JCGM 106).

Decision rule: guarded rejection: an item is rejected only if it is clearly outside the limit (e.g. in law enforcement or anti-doping, where a false accusation must be avoided).


#### Finally: what the source expects

Reference: Eurachem/CITAC 2021, Example B2: k = 1.86, g = 4.1 ng/g, acceptance limit 204.1 ng/g; 203.7 ng/g compliant.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="eurachem-compliance-b3-banned-substance-lognormal-exact"></a>

### Banned substance at a 2 ng/g limit with u_rel = 35 %: lognormal PDF, exact s_G (Eurachem/CITAC compliance guide, Example B3)

`eurachem-compliance-b3-banned-substance-lognormal-exact`

#### What is this project about?

Upper limit 2 ng/g, relative standard uncertainty 35 %; non-compliant if the probability above the limit is at least 95 %; measured 3.3 ng/g.


#### Step 1. The idea: does the item meet the specification?

The measured quantity w must be at most 2 ng/g. Because every measured value is uncertain, a result close to a limit could be on either side of it. The program calculates the **probability of conformity** and applies a **decision rule** (JCGM 106).

Decision rule: guarded rejection: an item is rejected only if it is clearly outside the limit (e.g. in law enforcement or anti-doping, where a false accusation must be avoided).


#### Finally: what the source expects

Reference: Eurachem/CITAC 2021, Example B3: U_F = 1.78, g = 1.6 ng/g, acceptance limit 3.6 ng/g (lognormal), 3.2 ng/g (normal); 3.3 ng/g compliant with the lognormal PDF only.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="eurachem-compliance-b3-banned-substance-lognormal"></a>

### Banned substance at a 2 ng/g limit with u_rel = 35 %: lognormal PDF, s_G ≈ u_rel (Eurachem/CITAC compliance guide, Example B3)

`eurachem-compliance-b3-banned-substance-lognormal`

#### What is this project about?

Upper limit 2 ng/g, relative standard uncertainty 35 %; non-compliant if the probability above the limit is at least 95 %; measured 3.3 ng/g.


#### Step 1. The idea: does the item meet the specification?

The measured quantity w must be at most 2 ng/g. Because every measured value is uncertain, a result close to a limit could be on either side of it. The program calculates the **probability of conformity** and applies a **decision rule** (JCGM 106).

Decision rule: guarded rejection: an item is rejected only if it is clearly outside the limit (e.g. in law enforcement or anti-doping, where a false accusation must be avoided).


#### Finally: what the source expects

Reference: Eurachem/CITAC 2021, Example B3: U_F = 1.78, g = 1.6 ng/g, acceptance limit 3.6 ng/g (lognormal), 3.2 ng/g (normal); 3.3 ng/g compliant with the lognormal PDF only.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="eurachem-compliance-b3-banned-substance-normal"></a>

### Banned substance at a 2 ng/g limit with u_rel = 35 %: normal PDF with u = u_rel·L = 0.7 ng/g (Eurachem/CITAC compliance guide, Example B3)

`eurachem-compliance-b3-banned-substance-normal`

#### What is this project about?

Upper limit 2 ng/g, relative standard uncertainty 35 %; non-compliant if the probability above the limit is at least 95 %; measured 3.3 ng/g.


#### Step 1. The idea: does the item meet the specification?

The measured quantity w must be at most 2 ng/g. Because every measured value is uncertain, a result close to a limit could be on either side of it. The program calculates the **probability of conformity** and applies a **decision rule** (JCGM 106).

Decision rule: guarded rejection: an item is rejected only if it is clearly outside the limit (e.g. in law enforcement or anti-doping, where a false accusation must be avoided).


#### Finally: what the source expects

Reference: Eurachem/CITAC 2021, Example B3: U_F = 1.78, g = 1.6 ng/g, acceptance limit 3.6 ng/g (lognormal), 3.2 ng/g (normal); 3.3 ng/g compliant with the lognormal PDF only.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Guard bands from the test uncertainty ratio (Intermediate)

In calibration, acceptance limits are often derived from the ratio of the tolerance to the measurement uncertainty (TUR). Three common guard-band methods are compared together with the risks they lead to.

<a id="tur-guard-band-dobbert-tur2-5"></a>

### Acceptance limits from the test uncertainty ratio: Dobbert's managed guard band (method 6), TUR = 2.5, global risks for a 95 % in-tolerance process

`tur-guard-band-dobbert-tur2-5`

#### What is this project about?

Tolerance ±1 (normalised), U₉₅ = 2u = 0.4, so TUR = 1/U₉₅ = 2.5. GBF = 1 − M/TUR, M = 1.04 − exp(0.38 ln TUR − 0.54). The process of the items is normal about the centre with 95 % of it within tolerance (s.d. 0.510213).


#### Step 1. The idea: does the item meet the specification?

The measured quantity Normalised deviation from nominal must be between -1 and 1. Because every measured value is uncertain, a result close to a limit could be on either side of it. The program calculates the **probability of conformity** and applies a **decision rule** (JCGM 106).

Information about the production process (the spread of items before measurement) is also used, so that the **global risks** can be calculated: the fraction of all accepted items that are actually bad (consumer's risk) and of rejected items that are actually good (producer's risk).


#### Finally: what the source expects

Reference: Suncal 1.7.1: guard-band factor and PFA_norm/PFR_norm(itp = 0.95, TUR, GB) for the global risks.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="tur-guard-band-rp10-tur2-0"></a>

### Acceptance limits from the test uncertainty ratio: NCSLI RP-10 guard band, TUR = 2.0, global risks for a 95 % in-tolerance process

`tur-guard-band-rp10-tur2-0`

#### What is this project about?

Tolerance ±1 (normalised), U₉₅ = 2u = 0.5, so TUR = 1/U₉₅ = 2.0. GBF = 1.25 − 1/TUR (TUR ≤ 4). The process of the items is normal about the centre with 95 % of it within tolerance (s.d. 0.510213).


#### Step 1. The idea: does the item meet the specification?

The measured quantity Normalised deviation from nominal must be between -1 and 1. Because every measured value is uncertain, a result close to a limit could be on either side of it. The program calculates the **probability of conformity** and applies a **decision rule** (JCGM 106).

Information about the production process (the spread of items before measurement) is also used, so that the **global risks** can be calculated: the fraction of all accepted items that are actually bad (consumer's risk) and of rejected items that are actually good (producer's risk).


#### Finally: what the source expects

Reference: Suncal 1.7.1: guard-band factor and PFA_norm/PFR_norm(itp = 0.95, TUR, GB) for the global risks.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="tur-guard-band-rss-tur1-5"></a>

### Acceptance limits from the test uncertainty ratio: root-sum-square guard band, TUR = 1.5, global risks for a 95 % in-tolerance process

`tur-guard-band-rss-tur1-5`

#### What is this project about?

Tolerance ±1 (normalised), U₉₅ = 2u = 0.6667, so TUR = 1/U₉₅ = 1.5. GBF = √(1 − 1/TUR²). The process of the items is normal about the centre with 95 % of it within tolerance (s.d. 0.510213).


#### Step 1. The idea: does the item meet the specification?

The measured quantity Normalised deviation from nominal must be between -1 and 1. Because every measured value is uncertain, a result close to a limit could be on either side of it. The program calculates the **probability of conformity** and applies a **decision rule** (JCGM 106).

Information about the production process (the spread of items before measurement) is also used, so that the **global risks** can be calculated: the fraction of all accepted items that are actually bad (consumer's risk) and of rejected items that are actually good (producer's risk).


#### Finally: what the source expects

Reference: Suncal 1.7.1: guard-band factor and PFA_norm/PFR_norm(itp = 0.95, TUR, GB) for the global risks.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Censored results ("less than") (Intermediate)

Some laboratories report "< 0.015" instead of a number. Three ways of handling them are compared: delete them, ignore the sign, or use half the limit.

<a id="iso13528-e1-censored-deleted"></a>

### Censored results in a PT round: censored results deleted (ISO 13528 E.1)

`iso13528-e1-censored-deleted`

#### What is this project about?

ISO 13528:2022 E.1: 23 results of which five are ‘less than’ values; the robust average and standard deviation of Algorithm A depend on how the censored results are treated.


#### Step 1. The idea: comparing laboratories

In a **proficiency test**, the same material is sent to 23 laboratories, which all measure **result**. Each laboratory then gets a score that tells whether its result agrees with the assigned value.

Assigned value: the robust average of the participants' results (Algorithm A, which limits the influence of outliers). The standard deviation for proficiency assessment σ_pt decides how far a result may be from the assigned value.


#### Step 2. The scores

- **z = (x − x_pt)/σ_pt**: |z| ≤ 2 satisfactory, 2 < |z| < 3 questionable, |z| ≥ 3 unsatisfactory.
- **ζ and E_n** also use the uncertainty that each laboratory reported; they check whether the laboratory's own uncertainty is realistic.


#### Finally: what the source expects

Reference: ISO 13528:2022 Table E.1 (x* = 26.81, s* = 5.29).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="iso13528-e1-censored-half"></a>

### Censored results in a PT round: 0.5 × the limit (ISO 13528 E.1)

`iso13528-e1-censored-half`

#### What is this project about?

ISO 13528:2022 E.1: 23 results of which five are ‘less than’ values; the robust average and standard deviation of Algorithm A depend on how the censored results are treated.


#### Step 1. The idea: comparing laboratories

In a **proficiency test**, the same material is sent to 23 laboratories, which all measure **result**. Each laboratory then gets a score that tells whether its result agrees with the assigned value.

Assigned value: the robust average of the participants' results (Algorithm A, which limits the influence of outliers). The standard deviation for proficiency assessment σ_pt decides how far a result may be from the assigned value.


#### Step 2. The scores

- **z = (x − x_pt)/σ_pt**: |z| ≤ 2 satisfactory, 2 < |z| < 3 questionable, |z| ≥ 3 unsatisfactory.
- **ζ and E_n** also use the uncertainty that each laboratory reported; they check whether the laboratory's own uncertainty is realistic.


#### Finally: what the source expects

Reference: ISO 13528:2022 Table E.1 (x* = 23.95, s* = 8.60).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="iso13528-e1-censored-ignored"></a>

### Censored results in a PT round: ‘<’ ignored (ISO 13528 E.1)

`iso13528-e1-censored-ignored`

#### What is this project about?

ISO 13528:2022 E.1: 23 results of which five are ‘less than’ values; the robust average and standard deviation of Algorithm A depend on how the censored results are treated.


#### Step 1. The idea: comparing laboratories

In a **proficiency test**, the same material is sent to 23 laboratories, which all measure **result**. Each laboratory then gets a score that tells whether its result agrees with the assigned value.

Assigned value: the robust average of the participants' results (Algorithm A, which limits the influence of outliers). The standard deviation for proficiency assessment σ_pt decides how far a result may be from the assigned value.


#### Step 2. The scores

- **z = (x − x_pt)/σ_pt**: |z| ≤ 2 satisfactory, 2 < |z| < 3 questionable, |z| ≥ 3 unsatisfactory.
- **ζ and E_n** also use the uncertainty that each laboratory reported; they check whether the laboratory's own uncertainty is realistic.


#### Finally: what the source expects

Reference: ISO 13528:2022 Table E.1 (x* = 26.01, s* = 7.23).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Homogeneity and stability (Intermediate)

Before a proficiency test, the provider must show that all units of the material are the same and stay the same. An analysis of variance on replicate measurements answers this.

<a id="iso13528-e2-arsenic-homogeneity"></a>

### Arsenic in chocolate: homogeneity and stability of PT items (ISO 13528 E.2)

`iso13528-e2-arsenic-homogeneity`

#### What is this project about?

ISO 13528:2022 E.2: between-sample standard deviation from 10 items measured in duplicate, compared with 0.3σ_pt, and a stability check after six weeks at 60 °C.


#### Step 1. The idea: comparing laboratories

**Homogeneity**: several units of the material are measured in replicate. An analysis of variance separates the variation between units from the repeatability of the measurement; the between-unit standard deviation must be small compared with σ_pt.

**Stability**: units measured at the start and at the end of the round must agree.


#### Finally: what the source expects

Reference: ISO 13528:2022 E.2 (Tables E.2 and E.3 and the summary statistics printed with them).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## An assigned value from a CRM (Intermediate)

The assigned value comes from a comparison with a certified reference material, and its uncertainty is part of the evaluation.

<a id="iso13528-e5-los-angeles-reference-value"></a>

### Assigned value from a comparison with a CRM: Los Angeles value of aggregates (ISO 13528 E.5)

`iso13528-e5-los-angeles-reference-value`

#### What is this project about?

ISO 13528:2022 E.5 (7.5): the proficiency test item and a similar CRM are tested in duplicate on 20 samples; x_pt is the certified value plus the average difference, u(x_pt) combines the CRM uncertainty and the standard uncertainty of the difference.


#### Step 1. What do we measure, and how is it calculated?

The result is **x_pt** in LA units. It is not measured directly: it is calculated from the input quantities with the measurement model

```
x_pt = x_CRM + d
```


#### Step 2. The input quantities and their uncertainties

The model has 2 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **x_CRM**: certified value of the CRM. Best value 21.62 LA units with standard uncertainty u = 0.26 LA units (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **d**: average difference PT item − CRM over 20 samples. Measured 20 times (a **Type A** evaluation, from statistics). The estimate is the mean 1.72750 LA units; the readings scatter with a standard deviation s = 1.07 LA units, and the standard uncertainty of the mean is u = s/√n = 1.07/√20 = 0.239 LA units, with ν = n − 1 = 19 degrees of freedom.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 23.35 LA units
- standard uncertainty u = 0.35 LA units

Reference: ISO 13528:2022 E.5 (x_pt = 23.35, u(x_pt) = 0.35 LA units; Table E.8: d = 1.73, s = 1.07, u(d) = 0.24).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Volume of a storage tank (Intermediate)

A product of powers (π r² h). The relative uncertainties combine simply, and the Monte Carlo method confirms the GUM result.

<a id="possolo-meija-2022-storage-tank-volume"></a>

### Volume of a cylindrical storage tank: product of powers, Gauss's formula and Monte Carlo (Possolo & Meija 2022)

`possolo-meija-2022-storage-tank-volume`

#### What is this project about?

V = πR²H with R = 8.40(3) m and H = 32.50(7) m.


#### Step 1. What do we measure, and how is it calculated?

The result is **V** in m³ (Internal volume of a cylindrical storage tank). It is not measured directly: it is calculated from the input quantities with the measurement model

```
V = pi * R^2 * H
```


#### Step 2. The input quantities and their uncertainties

The model has 2 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **R**: Radius, from the diameter of the roof measured with a tape. Best value 8.4 m with standard uncertainty u = 0.03 m (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **H**: Height, with a plumb line from the edge of the roof. Best value 32.5 m with standard uncertainty u = 0.07 m (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 7204 m³
- standard uncertainty u = 54 m³

Reference: Possolo & Meija (2022), pp. 21–24: V = 7204 m³, give or take 54 m³ (Gauss's formula and Monte Carlo, K = 10⁶).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## A Wheatstone bridge (Intermediate)

A model that is not a simple product: the sensitivity coefficients must be derived properly. The program does this symbolically.

<a id="possolo-meija-2022-wheatstone-bridge"></a>

### Wheatstone bridge: a measurement model that is not a product of powers (Possolo & Meija 2022)

`possolo-meija-2022-wheatstone-bridge`

#### What is this project about?

At balance R_U/R_G = R_F/R_EH, with R_EH = 1/(R_E⁻¹ + R_H⁻¹) the joint resistance of E and H in parallel, so R_U = R_G R_F (R_E⁻¹ + R_H⁻¹).


#### Step 1. What do we measure, and how is it calculated?

The result is **R_U** in Ω (Resistance of the unknown resistor U of a balanced Wheatstone bridge). It is not measured directly: it is calculated from the input quantities with the measurement model

```
R_U = R_G * R_F * (1/R_E + 1/R_H)
```


#### Step 2. The input quantities and their uncertainties

The model has 4 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **R_E**: Decade resistor E (EICO, 0.5 %). Best value 951 Ω with standard uncertainty u = 5 Ω (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **R_F**: Fixed resistor F. Best value 997 Ω with standard uncertainty u = 5 Ω (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **R_G**: Decade resistor G (General Radio, 0.05 %). Best value 909 Ω with standard uncertainty u = 0.5 Ω (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **R_H**: Resistance substitution box H, in parallel with E. Best value 225200 Ω with standard uncertainty u = 2300 Ω (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 957 Ω
- standard uncertainty u = 7 Ω

Reference: Possolo & Meija (2022), p. 25: R_U = 957 Ω, u(R_U) ≈ 7 Ω; contributions F 48 %, G 0.6 %, E 52 %, H 0.004 %.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Isotopic abundances (Intermediate)

The abundances of the silicon isotopes from two measured isotope ratios. The results are correlated because they must add up to 1.

<a id="iupac2024-4b-silicon-isotopic-abundances"></a>

### Isotopic abundances of silicon from two measured isotope amount ratios (IUPAC 2024, Example 4B)

`iupac2024-4b-silicon-isotopic-abundances`

#### What is this project about?

x₂₈ = 1/(1 + R₂₉/₂₈ + R₃₀/₂₈), x₂₉ = R₂₉/₂₈·x₂₈, x₃₀ = R₃₀/₂₈·x₂₈: three outputs from two inputs, with the covariance matrix Σ_y = JΣ_xJᵀ (Formula 3).


#### Step 1. What do we measure, and how is it calculated?

This project has 3 results (output quantities) that are calculated together from the same inputs. Each has its own equation:

- **x28** mol/mol: `x28 = 1/(1 + R29 + R30)`
- **x29** mol/mol: `x29 = R29/(1 + R29 + R30)`
- **x30** mol/mol: `x30 = R30/(1 + R29 + R30)`


#### Step 2. The input quantities and their uncertainties

The model has 2 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **R29**: Amount ratio n(²⁹Si)/n(²⁸Si), Mixture III. Best value 0.0500657 mol/mol with standard uncertainty u = 0.0000025 mol/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **R30**: Amount ratio n(³⁰Si)/n(²⁸Si), Mixture III. Best value 0.0329035 mol/mol with standard uncertainty u = 0.0000034 mol/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.

It also uses the **Monte Carlo method**: it draws 200 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 200 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

Reference: IUPAC 2024, Example 4B: x₂₈ = 0.923 3873(36), x₂₉ = 0.046 2300(22), x₃₀ = 0.030 3827(30) mol/mol.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## A purity described by a beta distribution (Intermediate)

The cadmium standard again, now with the purity as a beta distribution that cannot exceed 100 %. Monte Carlo handles such a distribution naturally.

<a id="iupac2024-4h-cadmium-standard-beta-mcm"></a>

### Cadmium calibration standard by Monte Carlo with a beta distribution for the purity (IUPAC 2024, Example 4H; QUAM A1)

`iupac2024-4h-cadmium-standard-beta-mcm`

#### What is this project about?

γ_Cd = w_Cd m_Cd/V with m_Cd = 100.28(5) mg, w_Cd = 0.999 900(58) g/g and V = 100.00(7) mL. The purity, bounded by 1 g/g, is modelled by a beta distribution, the volume by a triangular one.


#### Step 1. What do we measure, and how is it calculated?

The result is **gamma_Cd** in mg/mL (Mass concentration of cadmium in the calibration standard). It is not measured directly: it is calculated from the input quantities with the measurement model

```
gamma_Cd = w_Cd * m_Cd / V
```


#### Step 2. The input quantities and their uncertainties

The model has 3 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **m_Cd**: Mass of cadmium (Gaussian; the truncation at 0 mg has no effect). Best value 100.28 mg with standard uncertainty u = 0.05 mg (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **w_Cd**: Purity of the cadmium, beta distribution on [0, 1] with the stated mean and s.d.. Beta distribution between 0 and 1 g/g with mean 0.9999 and u = 0.000058 g/g (a quantity confined to an interval).
3. **V**: Volume of the flask, symmetric triangular with s.d. 0.07 mL. The value lies between 99.8285357 and 100.17 mL, and values near the centre 100 are more likely than values near the limits (triangular distribution). u = a/√6 = 0.1714643/√6 = 0.0700 mL.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 1.0027 mg/mL
- standard uncertainty u = 0.0009 mg/mL

Reference: IUPAC 2024, Example 4H: γ = 1.0027 mg/mL, u = 0.0009 mg/mL (Gauss and Monte Carlo, K = 10⁷), U95 = 0.0017 mg/mL (half the 2.5–97.5 % percentile range). Percentiles from numpy (10⁷ draws): 1.001 03 and 1.004 37 mg/mL.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## An asymmetric interval below 1 (Intermediate)

The purity of benzoic acid is close to 1 and cannot exceed it, so its interval is not symmetric. A beta distribution describes it.

<a id="iupac2024-5c-benzoic-acid-purity-beta"></a>

### Purity of benzoic acid (NIST PS1) expressed as a beta distribution: an asymmetric interval below 1 g/g (IUPAC 2024, Example 5C)

`iupac2024-5c-benzoic-acid-purity-beta`

#### What is this project about?

The state of knowledge about the mass fraction is a beta distribution with α = 88 873.7 and β = 7.11; its standard deviation is the standard uncertainty, and credible intervals follow from its quantiles.


#### Step 1. What do we measure, and how is it calculated?

The result is **w_BA** in g/g (Mass fraction of benzoic acid in NIST PS1). It is not measured directly: it is calculated from the input quantities with the measurement model

```
w_BA = w
```


#### Step 2. The input quantities and their uncertainties

The model has 1 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **w**: Beta distribution that approximates the Bayesian posterior of the purity. Beta distribution between 0 and 1 g/g (a quantity confined to an interval, possibly asymmetric, such as a purity close to 1): mean 0.999920 g/g, u = 3.00 × 10⁻⁵ g/g.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 0.99992 g/g
- standard uncertainty u = 0.00003 g/g

Reference: IUPAC 2024, Example 5C: beta distribution with α = 88 873.7 and β = 7.11; mean and s.d. 0.999 920 and 0.000 030 g/g; quantiles 0.999 851 and 0.999 968 g/g (scipy).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## End-gauge calibration (GUM H.1) (Intermediate)

The famous example of the GUM itself. It uses degrees of freedom for Type B terms and the Welch–Satterthwaite formula to choose the coverage factor.

<a id="gum-h1-end-gauge"></a>

### End-gauge calibration (GUM H.1)

`gum-h1-end-gauge`

#### What is this project about?

JCGM 100 Annex H.1: calibration of a 50 mm end gauge; Welch–Satterthwaite effective degrees of freedom and k for 99 %.


#### Step 1. What do we measure, and how is it calculated?

The result is **l** in nm (Length of the end gauge). It is not measured directly: it is calculated from the input quantities with the measurement model

```
l = l_s + d - l_s*(d_alpha*theta + alpha_s*d_theta)
```

To keep the equation readable, 1 intermediate quantities are calculated first:

- `theta = theta_bar + Delta`


#### Step 2. The input quantities and their uncertainties

The model has 7 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **l_s**: Length of the standard (calibration certificate). Best value 50000623 nm with standard uncertainty u = 25 nm (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings). This standard uncertainty is itself only known roughly: it has 18 degrees of freedom.
2. **d**: Measured length difference (mean of 5 readings, comparator). Best value 215 nm with standard uncertainty u = 9.7 nm (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings). This standard uncertainty is itself only known roughly: it has 25.6 degrees of freedom.
3. **alpha_s**: Thermal expansion coefficient of the standard. Only limits are known: the value lies between 0.0000095 and 0.0000135 1/°C (0.0000115 ± 0.000002), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.000002/√3 = 1.15 × 10⁻⁶ 1/°C.
4. **theta_bar**: Mean temperature deviation of the bed. Best value -0.1 °C with standard uncertainty u = 0.2 °C (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
5. **Delta**: Cyclic temperature variation. The value swings between -0.5 and 0.5 °C and is most often near the limits (U-shaped or arcsine distribution; e.g. a temperature cycling around a set point, or a mismatch phase). u = a/√2 = 0.5/√2 = 0.354 °C.
6. **d_alpha**: Difference of expansion coefficients. Only limits are known: the value lies between -0.000001 and 0.000001 1/°C (0 ± 0.000001), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.000001/√3 = 5.77 × 10⁻⁷ 1/°C. This standard uncertainty is itself only known roughly: it has 50 degrees of freedom.
7. **d_theta**: Temperature difference of the gauges. Only limits are known: the value lies between -0.05 and 0.05 °C (0 ± 0.05), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.05/√3 = 0.0289 °C. This standard uncertainty is itself only known roughly: it has 2 degrees of freedom.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It chooses the coverage factor k for a coverage probability of 99.0 %, using the effective degrees of freedom (Welch–Satterthwaite), and gives the **expanded uncertainty** U = k·u_c.

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 50000838 nm
- standard uncertainty u = 32 nm
- coverage factor k = 2.92
- expanded uncertainty U = 93 nm

Reference: JCGM 100:2008 H.1.4–H.1.6 (u_c = 32 nm, ν_eff = 16.7, k = 2.92, U = 93 nm).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Correlated inputs (GUM H.2) (Intermediate)

Voltage, current and phase are measured simultaneously, so their means are correlated. Resistance, reactance and impedance are calculated with and without the correlation terms.

<a id="gum-h2-impedance"></a>

### Impedance magnitude from correlated V and I (GUM H.2)

`gum-h2-impedance`

#### What is this project about?

GUM H.2: the impedance magnitude Z = V/I from correlated simultaneous measurements of voltage, current and phase angle.


#### Step 1. What do we measure, and how is it calculated?

The result is **Z** in Ω (The impedance magnitude z = v/i). It is not measured directly: it is calculated from the input quantities with the measurement model

```
Z = 1000 * V / I
```


#### Step 2. The input quantities and their uncertainties

The model has 3 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **V**: Mean of five voltage readings. Best value 4.999 V with standard uncertainty u = 0.0032094 V (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **I**: Mean of five current readings. Best value 19.661 mA with standard uncertainty u = 0.0094710 mA (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **phi**: Mean of five phase-angle readings. Best value 1.04446 rad with standard uncertainty u = 0.00075206 rad (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).

**Correlated inputs.** Some inputs are not independent, because a common effect (the same standard, the same instrument, the same temperature) acts on both. The correlation coefficient r tells how strongly; the program adds the corresponding term to the combined uncertainty:

- V ↔ I: r = -0.355311
- V ↔ phi: r = 0.857624
- I ↔ phi: r = -0.645111


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 254.26 Ω
- standard uncertainty u = 0.236 Ω

Reference: JCGM 100:2008 Table H.3: Z = 254.26 Ω, u_c = 0.236 Ω.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="gum-h2-reactance"></a>

### Reactance from correlated V, I and φ (GUM H.2)

`gum-h2-reactance`

#### What is this project about?

GUM H.2: the reactance X = (V/I) sin φ from correlated simultaneous measurements of voltage, current and phase angle.


#### Step 1. What do we measure, and how is it calculated?

The result is **X** in Ω (The reactance x = (v/i) sin φ). It is not measured directly: it is calculated from the input quantities with the measurement model

```
X = 1000 * V / I * sin(phi)
```


#### Step 2. The input quantities and their uncertainties

The model has 3 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **V**: Mean of five voltage readings. Best value 4.999 V with standard uncertainty u = 0.0032094 V (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **I**: Mean of five current readings. Best value 19.661 mA with standard uncertainty u = 0.0094710 mA (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **phi**: Mean of five phase-angle readings. Best value 1.04446 rad with standard uncertainty u = 0.00075206 rad (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).

**Correlated inputs.** Some inputs are not independent, because a common effect (the same standard, the same instrument, the same temperature) acts on both. The correlation coefficient r tells how strongly; the program adds the corresponding term to the combined uncertainty:

- V ↔ I: r = -0.355311
- V ↔ phi: r = 0.857624
- I ↔ phi: r = -0.645111


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 219.847 Ω
- standard uncertainty u = 0.295 Ω

Reference: JCGM 100:2008 Table H.3: X = 219.847 Ω, u_c = 0.295 Ω.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="gum-h2-resistance"></a>

### Resistance from correlated V, I and φ (GUM H.2)

`gum-h2-resistance`

#### What is this project about?

GUM H.2: the resistance R = (V/I) cos φ from correlated simultaneous measurements of voltage, current and phase angle.


#### Step 1. What do we measure, and how is it calculated?

The result is **R** in Ω (The resistance r = (v/i) cos φ). It is not measured directly: it is calculated from the input quantities with the measurement model

```
R = 1000 * V / I * cos(phi)
```


#### Step 2. The input quantities and their uncertainties

The model has 3 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **V**: Mean of five voltage readings. Best value 4.999 V with standard uncertainty u = 0.0032094 V (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **I**: Mean of five current readings. Best value 19.661 mA with standard uncertainty u = 0.0094710 mA (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **phi**: Mean of five phase-angle readings. Best value 1.04446 rad with standard uncertainty u = 0.00075206 rad (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).

**Correlated inputs.** Some inputs are not independent, because a common effect (the same standard, the same instrument, the same temperature) acts on both. The correlation coefficient r tells how strongly; the program adds the corresponding term to the combined uncertainty:

- V ↔ I: r = -0.355311
- V ↔ phi: r = 0.857624
- I ↔ phi: r = -0.645111


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 127.732 Ω
- standard uncertainty u = 0.071 Ω

Reference: JCGM 100:2008 Table H.3: R = 127.732 Ω, u_c = 0.071 Ω.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Examples of the NIST Uncertainty Machine (Intermediate)

Examples from the manual of the NIST Uncertainty Machine: a ratio with a wide triangular divisor, an end gauge, a resistance and the Stefan–Boltzmann constant. They compare GUM and Monte Carlo results.

<a id="num-10-resistance"></a>

### Resistance with the rounded values of GUM Table H.2 (NIST Uncertainty Machine §10)

`num-10-resistance`

#### What is this project about?

GUM H.2: the resistance R = (V/I) cos φ from correlated simultaneous measurements of voltage, current and phase angle.


#### Step 1. What do we measure, and how is it calculated?

The result is **R** in Ω (The resistance r = (v/i) cos φ). It is not measured directly: it is calculated from the input quantities with the measurement model

```
R = 1000 * V / I * cos(phi)
```


#### Step 2. The input quantities and their uncertainties

The model has 3 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **V**: Mean of five voltage readings. Best value 4.999 V with standard uncertainty u = 0.0032 V (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **I**: Mean of five current readings. Best value 19.661 mA with standard uncertainty u = 0.0095 mA (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **phi**: Mean of five phase-angle readings. Best value 1.04446 rad with standard uncertainty u = 0.00075 rad (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).

**Correlated inputs.** Some inputs are not independent, because a common effect (the same standard, the same instrument, the same temperature) acts on both. The correlation coefficient r tells how strongly; the program adds the corresponding term to the combined uncertainty:

- V ↔ I: r = -0.36
- V ↔ phi: r = 0.86
- I ↔ phi: r = -0.65


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 127.732 Ω
- standard uncertainty u = 0.07 Ω

Reference: GUM: law of propagation with the rounded inputs (0.0700 Ω); NIST Uncertainty Machine manual §10 (M = 10⁶): Gauss's formula u = 0.07 Ω; mean 127.732 Ω, sd 0.0699 Ω, 95 % interval (127.595, 127.869) Ω.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="num-3-ratio-abc"></a>

### Product and ratio y = ab/c with a wide triangular divisor (NIST Uncertainty Machine §3)

`num-3-ratio-abc`

#### What is this project about?

NUM manual §3: Gaussian a, rectangular b and a triangular divisor c with a 30 % relative uncertainty; the first-order GUM result is far from the Monte Carlo result.


#### Step 1. What do we measure, and how is it calculated?

The result is **y** (Output quantity). It is not measured directly: it is calculated from the input quantities with the measurement model

```
y = a * b / c
```


#### Step 2. The input quantities and their uncertainties

The model has 3 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **a**: Gaussian factor. Best value 32 with standard uncertainty u = 0.5 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **b**: Rectangular factor. Only limits are known: the value lies between 0.8566987 and 0.9433013 (0.9 ± 0.0433013), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.0433013/√3 = 0.0250.
3. **c**: Triangular divisor. The value lies between 0.26515 and 1.734847, and values near the centre 1 are more likely than values near the limits (triangular distribution). u = a/√6 = 0.734847/√6 = 0.300.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.

It also uses the **Monte Carlo method**: it draws 2 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 2 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 28.8
- standard uncertainty u = 8.7

Reference: NIST Uncertainty Machine manual §3, Figure 1 (Gauss's formula y = 28.8, u = 8.7; Monte Carlo, M = 10⁶: mean 32.2, sd 13, 95 % interval (18.2, 67)).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="num-8-end-gauge-gaussian"></a>

### End-gauge calibration with Gaussian inputs (NIST Uncertainty Machine §8)

`num-8-end-gauge-gaussian`

#### What is this project about?

NUM manual §8: GUM H.1 with the six inputs of the GUM's first-order budget, all modelled as Gaussian; the Monte Carlo method finds the second-order contributions that Gauss's formula misses.


#### Step 1. What do we measure, and how is it calculated?

The result is **l** in nm (Length of the end gauge at 20 °C). It is not measured directly: it is calculated from the input quantities with the measurement model

```
l = l_S + d - l_S * (d_alpha * theta + alpha_S * d_theta)
```


#### Step 2. The input quantities and their uncertainties

The model has 6 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **l_S**: Length of the reference standard. Best value 50000623 nm with standard uncertainty u = 25 nm (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **d**: Measured difference in length. Best value 215 nm with standard uncertainty u = 9.7 nm (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **d_alpha**: Difference of the expansion coefficients. Best value 0 1/°C with standard uncertainty u = 5.8e-7 1/°C (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **theta**: Deviation of the temperature from 20 °C. Best value -0.1 °C with standard uncertainty u = 0.41 °C (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
5. **alpha_S**: Expansion coefficient of the standard. Best value 0.0000115 1/°C with standard uncertainty u = 0.0000012 1/°C (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
6. **d_theta**: Temperature difference between the gauges. Best value 0 °C with standard uncertainty u = 0.029 °C (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It chooses the coverage factor k for a coverage probability of 99.0 %, using the effective degrees of freedom (Welch–Satterthwaite), and gives the **expanded uncertainty** U = k·u_c.

It also uses the **Monte Carlo method**: it draws 2 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 2 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 50000838 nm
- standard uncertainty u = 31.7 nm

Reference: NIST Uncertainty Machine manual §8, Figure 3 (Gauss's formula 50 000 838 nm, u = 31.7 nm; Monte Carlo, M = 10⁶: mean 50 000 837.9 nm, sd 33.9 nm, 99 % symmetric interval (50 000 749, 50 000 927) nm).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="num-11-stefan-boltzmann"></a>

### Stefan–Boltzmann constant from CODATA 2010 values (NIST Uncertainty Machine §11)

`num-11-stefan-boltzmann`

#### What is this project about?

NUM manual §11: σ = 32π⁵hR⁴R∞⁴/(15A_r(e)⁴M_u⁴c⁶α⁸) from the 2010 CODATA values; the molar gas constant R contributes almost all the uncertainty.


#### Step 1. What do we measure, and how is it calculated?

The result is **sigma** in 10⁻⁸ W m⁻² K⁻⁴ (Stefan–Boltzmann constant). It is not measured directly: it is calculated from the input quantities with the measurement model

```
sigma = 1e8 * 32 * pi^5 * h * R^4 * R_inf^4 / (15 * A_e^4 * (1e-3)^4 * 299792458^6 * alpha^8)
```


#### Step 2. The input quantities and their uncertainties

The model has 5 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **h**: Planck constant. Best value 6.6261 × 10⁻³⁴ J s with standard uncertainty u = 2.9e-41 J s (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **R**: Molar gas constant. Best value 8.3144621 J/(mol K) with standard uncertainty u = 0.0000075 J/(mol K) (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **R_inf**: Rydberg constant. Best value 10973732 1/m with standard uncertainty u = 0.000055 1/m (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **A_e**: Relative atomic mass of the electron. Best value 0.00054858 with standard uncertainty u = 2.2e-13 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
5. **alpha**: Fine-structure constant. Best value 0.0072974 with standard uncertainty u = 2.4e-12 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 5.6703725 10⁻⁸ W m⁻² K⁻⁴
- standard uncertainty u = 0.0000205 10⁻⁸ W m⁻² K⁻⁴

Reference: NIST Uncertainty Machine manual §11, Figure 7: σ = 5.670 3725 × 10⁻⁸, u(σ) = 2.05 × 10⁻¹³ W m⁻² K⁻⁴ (Gauss's formula and Monte Carlo), 95 % interval (5.670 332, 5.670 412 6) × 10⁻⁸ W m⁻² K⁻⁴.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Calibrating a thermocouple (Intermediate)

A thermocouple is compared with a reference thermocouple in a furnace. Two budgets: the temperature of the furnace and the emf of the unknown thermocouple.

<a id="ea402-s5-thermocouple-emf"></a>

### Thermocouple calibration: emf of the unknown

`ea402-s5-thermocouple-emf`

#### What is this project about?

EA-4/02 S5 (second budget): emf of the type N thermocouple under calibration at 1000 °C.


#### Step 1. What do we measure, and how is it calculated?

The result is **V_X** in µV (Emf of the thermocouple at 1000 °C). It is not measured directly: it is calculated from the input quantities with the measurement model

```
V_X = V_iX + V_iX1 + V_iX2 + V_R + V_LX + dt/0.026 - t_0X/0.039
```


#### Step 2. The input quantities and their uncertainties

The model has 7 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **V_iX**: Mean voltmeter reading. Best value 36248 µV with standard uncertainty u = 1.6 µV (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **V_iX1**: Voltmeter calibration. Best value 0 µV with standard uncertainty u = 1 µV (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **V_iX2**: Voltmeter resolution. Only limits are known: the value lies between -0.5 and 0.5 µV (0 ± 0.5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.5/√3 = 0.289 µV.
4. **V_R**: Contact parasitic voltages. Only limits are known: the value lies between -2 and 2 µV (0 ± 2), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 2/√3 = 1.15 µV.
5. **V_LX**: Extension leads. Only limits are known: the value lies between -5 and 5 µV (0 ± 5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 5/√3 = 2.89 µV.
6. **dt**: Calibration point minus furnace temperature. Best value -0.5 °C with standard uncertainty u = 0.641 °C (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
7. **t_0X**: Reference junction temperature. Only limits are known: the value lies between -0.1 and 0.1 °C (0 ± 0.1), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.1/√3 = 0.0577 °C.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 36229 µV
- standard uncertainty u = 25 µV
- expanded uncertainty U = 50 µV

Reference: EA-4/02 M:2022 S5.10–S5.12 (u(V_X) = 25.0 µV, U = 50 µV).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="ea402-s5-thermocouple-furnace"></a>

### Thermocouple calibration: furnace temperature

`ea402-s5-thermocouple-furnace`

#### What is this project about?

EA-4/02 S5 (first budget): temperature of a furnace at 1000 °C measured with a type N reference thermocouple.


#### Step 1. What do we measure, and how is it calculated?

The result is **t_X** in °C (Temperature of the furnace). It is not measured directly: it is calculated from the input quantities with the measurement model

```
t_X = t_S + 0.077*(V_iS1 + V_iS2 + V_R) - 0.077/0.189*t_0S + dt_S + dt_D + dt_F
```


#### Step 2. The input quantities and their uncertainties

The model has 8 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **t_S**: Temperature from the reference thermocouple. Best value 1000.5 °C with standard uncertainty u = 0.1 °C (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **V_iS1**: Voltmeter calibration. Best value 0 µV. The source (e.g. a certificate) gives the expanded uncertainty U = 2 µV with k = 2, so the standard uncertainty is u = U/k = 2/2 = 1.00 µV (normal distribution).
3. **V_iS2**: Voltmeter resolution. Only limits are known: the value lies between -0.5 and 0.5 µV (0 ± 0.5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.5/√3 = 0.289 µV.
4. **V_R**: Contact parasitic voltages. Only limits are known: the value lies between -2 and 2 µV (0 ± 2), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 2/√3 = 1.15 µV.
5. **t_0S**: Reference temperature of the cold junction. Only limits are known: the value lies between -0.1 and 0.1 °C (0 ± 0.1), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.1/√3 = 0.0577 °C.
6. **dt_S**: Calibration of the reference thermocouple. Best value 0 °C. The source (e.g. a certificate) gives the expanded uncertainty U = 0.3 °C with k = 2, so the standard uncertainty is u = U/k = 0.3/2 = 0.150 °C (normal distribution).
7. **dt_D**: Drift of the reference thermocouple. Only limits are known: the value lies between -0.3 and 0.3 °C (0 ± 0.3), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.3/√3 = 0.173 °C.
8. **dt_F**: Non-uniformity of the furnace. Only limits are known: the value lies between -1 and 1 °C (0 ± 1), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 1/√3 = 0.577 °C.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 1000.5 °C
- standard uncertainty u = 0.641 °C
- expanded uncertainty U = 1.3 °C

Reference: EA-4/02 M:2022 S5.9 (u(t_X) = 0.641 °C).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## A power sensor at 18 GHz (Intermediate)

A microwave power sensor: mismatch between sensor and source is a U-shaped (arcsine) term.

<a id="ea402-s6-power-sensor-18ghz"></a>

### Calibration of a power sensor at 18 GHz

`ea402-s6-power-sensor-18ghz`

#### What is this project about?

EA-4/02 S6: calibration factor of a thermistor power sensor against a standard sensor; mismatch terms are U-shaped.


#### Step 1. What do we measure, and how is it calculated?

The result is **K_X** (Calibration factor of the sensor). It is not measured directly: it is calculated from the input quantities with the measurement model

```
K_X = (K_S + dK_D) * M_Sr * M_Xc / (M_Sc * M_Xr) * p_Cr * p_Cc * p
```


#### Step 2. The input quantities and their uncertainties

The model has 9 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **K_S**: Calibration factor of the standard. Best value 0.957. The source (e.g. a certificate) gives the expanded uncertainty U = 0.011 with k = 2, so the standard uncertainty is u = U/k = 0.011/2 = 0.00550 (normal distribution).
2. **dK_D**: Drift of the standard. Only limits are known: the value lies between -0.003 and 0.001 (-0.001 ± 0.002), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.002/√3 = 0.00115.
3. **M_Sr**: Mismatch, standard, reference port. The value swings between 0.9992 and 1.0008 and is most often near the limits (U-shaped or arcsine distribution; e.g. a temperature cycling around a set point, or a mismatch phase). u = a/√2 = 0.0008/√2 = 0.000566.
4. **M_Sc**: Mismatch, standard, 18 GHz. The value swings between 0.986 and 1.014 and is most often near the limits (U-shaped or arcsine distribution; e.g. a temperature cycling around a set point, or a mismatch phase). u = a/√2 = 0.014000/√2 = 0.00990.
5. **M_Xr**: Mismatch, unknown, reference port. The value swings between 0.9992 and 1.0008 and is most often near the limits (U-shaped or arcsine distribution; e.g. a temperature cycling around a set point, or a mismatch phase). u = a/√2 = 0.0008/√2 = 0.000566.
6. **M_Xc**: Mismatch, unknown, 18 GHz. The value swings between 0.9832 and 1.0168 and is most often near the limits (U-shaped or arcsine distribution; e.g. a temperature cycling around a set point, or a mismatch phase). u = a/√2 = 0.016800/√2 = 0.0119.
7. **p_Cr**: Power meter, reference ratio. Best value 1 with standard uncertainty u = 0.00142 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
8. **p_Cc**: Power meter, 18 GHz ratio. Best value 1 with standard uncertainty u = 0.000142 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
9. **p**: Observed power ratio (3 measurements). Measured 3 times (a **Type A** evaluation, from statistics). The estimate is the mean 0.975988; the readings scatter with a standard deviation s = 0.00833, and the standard uncertainty of the mean is u = s/√n = 0.00833/√3 = 0.00481, with ν = n − 1 = 2 degrees of freedom.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 0.933
- standard uncertainty u = 0.01623
- expanded uncertainty U = 0.032

Reference: EA-4/02 M:2022 S6.12–S6.13 (u = 0.0162, U = 0.032, ν_eff ≈ 310).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## A step attenuator (Intermediate)

A 30 dB attenuator. Uncertainties in dB, mismatch terms and the repeatability of connections.

<a id="ea402-s7-step-attenuator-30db"></a>

### Calibration of a coaxial step attenuator (30 dB)

`ea402-s7-step-attenuator-30db`

#### What is this project about?

EA-4/02 S7: 30 dB step of a coaxial attenuator at 10 GHz, with mismatch from S-parameters.


#### Step 1. What do we measure, and how is it calculated?

The result is **L_X** in dB (Attenuation of the 30 dB step). It is not measured directly: it is calculated from the input quantities with the measurement model

```
L_X = L_S + dL_S + dL_D + dL_M + dL_K + dL_ib - dL_ia + dL_0b - dL_0a
```


#### Step 2. The input quantities and their uncertainties

The model has 9 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **L_S**: Observed attenuation (4 readings). Measured 4 times (a **Type A** evaluation, from statistics). The estimate is the mean 30.0403 dB; the readings scatter with a standard deviation s = 0.0183 dB, and the standard uncertainty of the mean is u = s/√n = 0.0183/√4 = 0.00913 dB, with ν = n − 1 = 3 degrees of freedom.
2. **dL_S**: Calibration of the standard. Best value 0.003 dB. The source (e.g. a certificate) gives the expanded uncertainty U = 0.005 dB with k = 2, so the standard uncertainty is u = U/k = 0.005/2 = 0.00250 dB (normal distribution).
3. **dL_D**: Drift of the standard. Only limits are known: the value lies between -0.002 and 0.002 dB (0 ± 0.002), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.002/√3 = 0.00115 dB.
4. **dL_M**: Mismatch (Eq. S7.2). Best value 0 dB with standard uncertainty u = 0.019788 dB (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
5. **dL_K**: Leakage. Only limits are known: the value lies between -0.003 and 0.003 dB (0 ± 0.003), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.003/√3 = 0.00173 dB.
6. **dL_ia**: Resolution, reference setting. Only limits are known: the value lies between -0.0005 and 0.0005 dB (0 ± 0.0005), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.0005/√3 = 0.000289 dB.
7. **dL_ib**: Resolution, attenuation setting. Only limits are known: the value lies between -0.0005 and 0.0005 dB (0 ± 0.0005), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.0005/√3 = 0.000289 dB.
8. **dL_0a**: Noise, reference setting. Best value 0 dB with standard uncertainty u = 0.002 dB (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
9. **dL_0b**: Noise, attenuation setting. Best value 0 dB with standard uncertainty u = 0.002 dB (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 30.043 dB
- standard uncertainty u = 0.0223 dB
- expanded uncertainty U = 0.045 dB

Reference: EA-4/02 M:2022 S7.12–S7.13 (u = 0.0223 dB, U = 0.045 dB).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## A ring gauge (Intermediate)

The diameter of a 90 mm ring gauge with its temperature correction, in two budgets.

<a id="ea402-s13-ring-gauge-90mm"></a>

### Calibration of a 90 mm ring gauge

`ea402-s13-ring-gauge-90mm`

#### What is this project about?

EA-4/02 S13: diameter of a ring gauge compared with a 40 mm setting ring on a one-coordinate measuring machine (µm relative to 90 mm).


#### Step 1. What do we measure, and how is it calculated?

The result is **d_X** in µm (Diameter deviation from 90 mm). It is not measured directly: it is calculated from the input quantities with the measurement model

```
d_X = d_S + dl + dl_0 + dl_i + dl_T + dl_P + dl_E + dl_A
```


#### Step 2. The input quantities and their uncertainties

The model has 8 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **d_S**: Setting ring diameter deviation (40 mm). Best value 0.7 µm. The source (e.g. a certificate) gives the expanded uncertainty U = 0.2 µm with k = 2, so the standard uncertainty is u = U/k = 0.2/2 = 0.100 µm (normal distribution).
2. **dl**: Observed diameter difference relative to 50 mm (5 readings). Measured 5 times (a **Type A** evaluation, from statistics). The estimate is the mean -0.464000 µm; the readings scatter with a standard deviation s = 0.328 µm, and the standard uncertainty of the mean is u = s/√n = 0.328/√5 = 0.147 µm, with ν = n − 1 = 4 degrees of freedom.
3. **dl_0**: Zeroing of the comparator (s_p(0) = 0.25 µm). Best value 0 µm with standard uncertainty u = 0.25 µm (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **dl_i**: Machine scale calibration and linearity. Only limits are known: the value lies between -0.375 and 0.375 µm (0 ± 0.375), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.375/√3 = 0.217 µm.
5. **dl_T**: Temperature correction (sub-budget). Best value 0 µm with standard uncertainty u = 0.148 µm (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
6. **dl_P**: Coaxiality (cosine) error. Best value -0.003704 µm with standard uncertainty u = 0.006525 µm (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
7. **dl_E**: Elastic deformation. Only limits are known: the value lies between -0.03 and 0.03 µm (0 ± 0.03), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.03/√3 = 0.0173 µm.
8. **dl_A**: Abbe error. Only limits are known: the value lies between -0.02 and 0.02 µm (0 ± 0.02), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.02/√3 = 0.0115 µm.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 0.23230 µm
- standard uncertainty u = 0.411 µm
- expanded uncertainty U = 0.82 µm

Reference: EA-4/02 M:2022 S13.10–S13.12 (u = 0.411 µm, U = 0.82 µm reported as 0.9 µm; the tolerance covers u(Δl) = 0.29 µm here vs 0.30 µm printed); estimate recomputed from the observations, 0.7 − 0.464 − 0.0037 µm (printed 90.000 25 mm, see notes).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="ea402-s13-ring-gauge-temperature"></a>

### Ring gauge calibration: temperature correction

`ea402-s13-ring-gauge-temperature`

#### What is this project about?

EA-4/02 S13: temperature sub-budget of the ring-gauge comparison; products of zero-mean temperature deviations with expansion coefficients.


#### Step 1. What do we measure, and how is it calculated?

The result is **dl_T** in µm (Temperature correction). It is not measured directly: it is calculated from the input quantities with the measurement model

```
dl_T = l_TA + l_TS - l_TX + l_TR
```

To keep the equation readable, 4 intermediate quantities are calculated first:

- `l_TA = (40000*(a_S - a_R) - 90000*(a_X - a_R))*dt_A`
- `l_TS = 40000*a_S*dt_S`
- `l_TX = 90000*a_X*dt_X`
- `l_TR = (40000 - 90000)*a_R*dt_R`


#### Step 2. The input quantities and their uncertainties

The model has 7 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **a_S**: Expansion coefficient of the setting ring. Only limits are known: the value lies between 0.0000105 and 0.0000125 1/K (0.0000115 ± 0.000001), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.000001/√3 = 5.77 × 10⁻⁷ 1/K.
2. **a_X**: Expansion coefficient of the ring gauge. Only limits are known: the value lies between 0.0000105 and 0.0000125 1/K (0.0000115 ± 0.000001), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.000001/√3 = 5.77 × 10⁻⁷ 1/K.
3. **a_R**: Expansion coefficient of the machine scale. Only limits are known: the value lies between 0.0000105 and 0.0000125 1/K (0.0000115 ± 0.000001), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.000001/√3 = 5.77 × 10⁻⁷ 1/K.
4. **dt_A**: Deviation of the mean temperature from 20 °C. Only limits are known: the value lies between -0.5 and 0.5 K (0 ± 0.5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.5/√3 = 0.289 K.
5. **dt_S**: Temperature difference, setting ring. Only limits are known: the value lies between -0.2 and 0.2 K (0 ± 0.2), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.2/√3 = 0.115 K.
6. **dt_X**: Temperature difference, ring gauge. Only limits are known: the value lies between -0.2 and 0.2 K (0 ± 0.2), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.2/√3 = 0.115 K.
7. **dt_R**: Temperature difference, scale. Only limits are known: the value lies between -0.2 and 0.2 K (0 ± 0.2), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.2/√3 = 0.115 K.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).
6. Because the model is clearly nonlinear, second-order terms are added (GUM 5.1.2 and its 2026 amendment).

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 0 µm
- standard uncertainty u = 0.15 µm

Reference: EA-4/02 M:2022 S13.5 (u(δl_T) = 0.15 µm).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## A power sensor (UKAS M3003) (Intermediate)

The British version of the power-sensor budget, with its mismatch term.

<a id="m3003-k2-power-sensor-18ghz"></a>

### Coaxial power sensor at 18 GHz (UKAS M3003 K.2)

`m3003-k2-power-sensor-18ghz`

#### What is this project about?

Calibration factor of an unknown power sensor by substitution against a standard sensor on a monitored source; four connections.


#### Step 1. What do we measure, and how is it calculated?

The result is **K_X** in % (Calibration factor of the unknown sensor). It is not measured directly: it is calculated from the input quantities with the measurement model

```
K_X = K_obs + dK_S + dD_S + d_DC + d_REF + M_1 + M_2 + M_3 + M_4
```


#### Step 2. The input quantities and their uncertainties

The model has 9 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **K_obs**: Observed calibration factor, four connections. Measured 4 times (a **Type A** evaluation, from statistics). The estimate is the mean 93.1550 %; the readings scatter with a standard deviation s = 0.741 %, and the standard uncertainty of the mean is u = s/√n = 0.741/√4 = 0.371 %, with ν = n − 1 = 3 degrees of freedom.
2. **dK_S**: Calibration factor of the standard (U = 1.1 %, k = 2). Best value 0 %. The source (e.g. a certificate) gives the expanded uncertainty U = 1.1 % with k = 2, so the standard uncertainty is u = U/k = 1.1/2 = 0.550 % (normal distribution).
3. **dD_S**: Drift of the standard since calibration. Only limits are known: the value lies between -0.2 and 0.2 % (0 ± 0.2), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.2/√3 = 0.115 %.
4. **d_DC**: Instrumentation linearity (U = 0.1 %, k = 2). Best value 0 %. The source (e.g. a certificate) gives the expanded uncertainty U = 0.1 % with k = 2, so the standard uncertainty is u = U/k = 0.1/2 = 0.0500 % (normal distribution).
5. **d_REF**: Stability of the 50 MHz reference. Only limits are known: the value lies between -0.2 and 0.2 % (0 ± 0.2), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.2/√3 = 0.115 %.
6. **M_1**: Mismatch, standard sensor at 50 MHz. The value swings between -0.08 and 0.08 % and is most often near the limits (U-shaped or arcsine distribution; e.g. a temperature cycling around a set point, or a mismatch phase). u = a/√2 = 0.08/√2 = 0.0566 %.
7. **M_2**: Mismatch, unknown sensor at 50 MHz. The value swings between -0.08 and 0.08 % and is most often near the limits (U-shaped or arcsine distribution; e.g. a temperature cycling around a set point, or a mismatch phase). u = a/√2 = 0.08/√2 = 0.0566 %.
8. **M_3**: Mismatch, standard sensor at 18 GHz. The value swings between -1.4 and 1.4 % and is most often near the limits (U-shaped or arcsine distribution; e.g. a temperature cycling around a set point, or a mismatch phase). u = a/√2 = 1.4/√2 = 0.990 %.
9. **M_4**: Mismatch, unknown sensor at 18 GHz. The value swings between -1.68 and 1.68 % and is most often near the limits (U-shaped or arcsine distribution; e.g. a temperature cycling around a set point, or a mismatch phase). u = a/√2 = 1.68/√2 = 1.19 %.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 200 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 200 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 93.16 %
- standard uncertainty u = 1.69 %
- expanded uncertainty U = 3.39 %

Reference: UKAS M3003 ed. 6 K.2.9–K.2.10: u_c = 1.69 %, U = 3.39 %; 93.2 % ± 3.4 %.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## An attenuator (UKAS M3003) (Intermediate)

A 30 dB attenuator at 10 GHz, with two ways of calculating the mismatch term.

<a id="m3003-k3-attenuator-30db-mismatch-e5"></a>

### 30 dB attenuator with the mismatch term from formula E5 (UKAS M3003 K.3, E.9.5)

`m3003-k3-attenuator-30db-mismatch-e5`

#### What is this project about?

The budget of K.3 with the mismatch standard deviation recomputed from the coefficients the example gives (finding F57).


#### Step 1. What do we measure, and how is it calculated?

The result is **A_X** in dB (Attenuation of the 30 dB step at 10 GHz). It is not measured directly: it is calculated from the input quantities with the measurement model

```
A_X = A_obs + dA_IF + dD_IF + dL_M + dR_D + dM + dA_L
```


#### Step 2. The input quantities and their uncertainties

The model has 7 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **A_obs**: Indicated attenuation A_b − A_a, four measurements. Measured 4 times (a **Type A** evaluation, from statistics). The estimate is the mean 30.0500 dB; the readings scatter with a standard deviation s = 0.0183 dB, and the standard uncertainty of the mean is u = s/√n = 0.0183/√4 = 0.00913 dB, with ν = n − 1 = 3 degrees of freedom.
2. **dA_IF**: Calibration of the reference IF attenuator (U = 0.005 dB, k = 2). Best value 0 dB. The source (e.g. a certificate) gives the expanded uncertainty U = 0.005 dB with k = 2, so the standard uncertainty is u = U/k = 0.005/2 = 0.00250 dB (normal distribution).
3. **dD_IF**: Drift of the IF attenuator. Only limits are known: the value lies between -0.002 and 0.002 dB (0 ± 0.002), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.002/√3 = 0.00115 dB.
4. **dL_M**: Mixer non-linearity (±0.002 dB per 10 dB). Only limits are known: the value lies between -0.006 and 0.006 dB (0 ± 0.006), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.006/√3 = 0.00346 dB.
5. **dR_D**: Resolution at the 0 dB and 30 dB readings, two rectangles combined. The value lies between -0.01 and 0.01 dB, and values near the centre 0 are more likely than values near the limits (triangular distribution). u = a/√6 = 0.01/√6 = 0.00408 dB.
6. **dM**: Mismatch (Harris and Warner, E.9.5). Best value 0 dB with standard uncertainty u = 0.016996 dB (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
7. **dA_L**: Signal leakage. Only limits are known: the value lies between -0.001 and 0.001 dB (0 ± 0.001), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.001/√3 = 0.000577 dB.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 200 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 200 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 30.05 dB
- standard uncertainty u = 0.020218 dB
- expanded uncertainty U = 0.040437 dB

Reference: Recomputed from formula E5 and the data of K.3.6 (research log F57); not a printed result.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="m3003-k3-attenuator-30db"></a>

### 30 dB coaxial attenuator at 10 GHz (UKAS M3003 K.3)

`m3003-k3-attenuator-30db`

#### What is this project about?

Calibration of a coaxial step attenuator by IF substitution; the mismatch term dominates.


#### Step 1. What do we measure, and how is it calculated?

The result is **A_X** in dB (Attenuation of the 30 dB step at 10 GHz). It is not measured directly: it is calculated from the input quantities with the measurement model

```
A_X = A_obs + dA_IF + dD_IF + dL_M + dR_D + dM + dA_L
```


#### Step 2. The input quantities and their uncertainties

The model has 7 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **A_obs**: Indicated attenuation A_b − A_a, four measurements. Measured 4 times (a **Type A** evaluation, from statistics). The estimate is the mean 30.0500 dB; the readings scatter with a standard deviation s = 0.0183 dB, and the standard uncertainty of the mean is u = s/√n = 0.0183/√4 = 0.00913 dB, with ν = n − 1 = 3 degrees of freedom.
2. **dA_IF**: Calibration of the reference IF attenuator (U = 0.005 dB, k = 2). Best value 0 dB. The source (e.g. a certificate) gives the expanded uncertainty U = 0.005 dB with k = 2, so the standard uncertainty is u = U/k = 0.005/2 = 0.00250 dB (normal distribution).
3. **dD_IF**: Drift of the IF attenuator. Only limits are known: the value lies between -0.002 and 0.002 dB (0 ± 0.002), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.002/√3 = 0.00115 dB.
4. **dL_M**: Mixer non-linearity (±0.002 dB per 10 dB). Only limits are known: the value lies between -0.006 and 0.006 dB (0 ± 0.006), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.006/√3 = 0.00346 dB.
5. **dR_D**: Resolution at the 0 dB and 30 dB readings, two rectangles combined. The value lies between -0.01 and 0.01 dB, and values near the centre 0 are more likely than values near the limits (triangular distribution). u = a/√6 = 0.01/√6 = 0.00408 dB.
6. **dM**: Mismatch (Harris and Warner, E.9.5). Best value 0 dB with standard uncertainty u = 0.022 dB (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
7. **dA_L**: Signal leakage. Only limits are known: the value lies between -0.001 and 0.001 dB (0 ± 0.001), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.001/√3 = 0.000577 dB.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 200 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 200 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 30.05 dB
- standard uncertainty u = 0.0245 dB
- expanded uncertainty U = 0.0491 dB

Reference: UKAS M3003 ed. 6 K.3.9–K.3.10: u_c = 0.0245 dB (0.024 57 unrounded), U = 0.0491 dB; 30.050 dB ± 0.049 dB.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## A gauge block (UKAS M3003) (Intermediate)

A 10 mm gauge block. One variant writes the temperature effects into the model itself, which shows the difference between a budget table and a measurement model.

<a id="m3003-k6-gauge-block-10mm-temperature-model"></a>

### Grade 2 gauge block of 10 mm with the temperature terms of its model (UKAS M3003 K.6, EA-4/02 S4)

`m3003-k6-gauge-block-10mm-temperature-model`

#### What is this project about?

The budget of K.6 with −L(α·Δt + Δα·ΔT) evaluated as the model states, as EA-4/02 S4 does for a 50 mm block (finding F58).


#### Step 1. What do we measure, and how is it calculated?

The result is **L_X** in mm (Length of the unknown gauge block). It is not measured directly: it is calculated from the input quantities with the measurement model

```
L_X = L_SdL + dL_D + dD_C + dC - L*(alpha*dt + d_alpha*dT) + L_V + dL_r
```


#### Step 2. The input quantities and their uncertainties

The model has 11 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **L_SdL**: L_S + ΔL: certified length of the standard plus the measured difference (only the sum is given, K.6.9); certificate U = 30 nm (k = 2). Best value 9.99994 mm. The source (e.g. a certificate) gives the expanded uncertainty U = 0.00003 mm with k = 2, so the standard uncertainty is u = U/k = 0.00003/2 = 1.50 × 10⁻⁵ mm (normal distribution).
2. **dL_D**: Drift of the standard (±15 nm, triangular). The value lies between -0.000015 and 0.000015 mm, and values near the centre 0 are more likely than values near the limits (triangular distribution). u = a/√6 = 0.000015/√6 = 6.12 × 10⁻⁶ mm.
3. **dD_C**: Comparator discrimination and linearity (±0.05 µm). Only limits are known: the value lies between -0.00005 and 0.00005 mm (0 ± 0.00005), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.00005/√3 = 2.89 × 10⁻⁵ mm.
4. **dC**: Difference in elastic compression (±0.005 µm). Only limits are known: the value lies between -0.000005 and 0.000005 mm (0 ± 0.000005), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.000005/√3 = 2.89 × 10⁻⁶ mm.
5. **L**: Nominal length. An exact value, 10 mm, with no uncertainty (for example a defined constant or a nominal value).
6. **alpha**: Mean expansion coefficient. An exact value, 0.0000115 1/°C, with no uncertainty (for example a defined constant or a nominal value).
7. **dt**: Temperature difference of the two blocks. Only limits are known: the value lies between -0.08 and 0.08 °C (0 ± 0.08), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.08/√3 = 0.0462 °C.
8. **d_alpha**: Difference of the expansion coefficients. The value lies between -0.000002 and 0.000002 1/°C, and values near the centre 0 are more likely than values near the limits (triangular distribution). u = a/√6 = 0.000002/√6 = 8.16 × 10⁻⁷ 1/°C.
9. **dT**: Deviation of the mean temperature from 20 °C. Only limits are known: the value lies between -0.2 and 0.2 °C (0 ± 0.2), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.2/√3 = 0.115 °C.
10. **L_V**: Length variation of the unknown block (±42 nm, triangular). The value lies between -0.000042 and 0.000042 mm, and values near the centre 0 are more likely than values near the limits (triangular distribution). u = a/√6 = 0.000042/√6 = 1.71 × 10⁻⁵ mm.
11. **dL_r**: Repeatability (s = 16 nm from 11 earlier measurements, one reading). Mean of 1 readings, 0 mm. Instead of the scatter of these few readings, a standard deviation s_p = 0.000016 mm determined earlier from more data (ν = 10) is used, so u = s_p/√n = 1.60 × 10⁻⁵ mm.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).
6. Because the model is clearly nonlinear, second-order terms are added (GUM 5.1.2 and its 2026 amendment).

It also uses the **Monte Carlo method**: it draws 200 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 200 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 9.99994 mm
- standard uncertainty u = 4.1028 × 10⁻⁵ mm
- expanded uncertainty U = 8.2055 × 10⁻⁵ mm

Reference: Recomputed from the data of K.6 with the model of K.6.1 (research log F58); not a printed result.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="m3003-k6-gauge-block-10mm"></a>

### Grade 2 gauge block of 10 mm (UKAS M3003 K.6)

`m3003-k6-gauge-block-10mm`

#### What is this project about?

Comparison with a standard gauge block; the budget as printed, with the two temperature effects added linearly into one triangular term.


#### Step 1. What do we measure, and how is it calculated?

The result is **L_X** in mm (Length of the unknown gauge block). It is not measured directly: it is calculated from the input quantities with the measurement model

```
L_X = L_SdL + dL_D + dD_C + dC + dT_SX + L_V + dL_r
```


#### Step 2. The input quantities and their uncertainties

The model has 7 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **L_SdL**: L_S + ΔL: certified length of the standard plus the measured difference (only the sum is given, K.6.9); certificate U = 30 nm (k = 2). Best value 9.99994 mm. The source (e.g. a certificate) gives the expanded uncertainty U = 0.00003 mm with k = 2, so the standard uncertainty is u = U/k = 0.00003/2 = 1.50 × 10⁻⁵ mm (normal distribution).
2. **dL_D**: Drift of the standard (±15 nm, triangular). The value lies between -0.000015 and 0.000015 mm, and values near the centre 0 are more likely than values near the limits (triangular distribution). u = a/√6 = 0.000015/√6 = 6.12 × 10⁻⁶ mm.
3. **dD_C**: Comparator discrimination and linearity (±0.05 µm). Only limits are known: the value lies between -0.00005 and 0.00005 mm (0 ± 0.00005), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.00005/√3 = 2.89 × 10⁻⁵ mm.
4. **dC**: Difference in elastic compression (±0.005 µm). Only limits are known: the value lies between -0.000005 and 0.000005 mm (0 ± 0.000005), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.000005/√3 = 2.89 × 10⁻⁶ mm.
5. **dT_SX**: Temperature effects, added linearly (±5.6 nm, triangular). The value lies between -0.0000056 and 0.0000056 mm, and values near the centre 0 are more likely than values near the limits (triangular distribution). u = a/√6 = 0.0000056/√6 = 2.29 × 10⁻⁶ mm.
6. **L_V**: Length variation of the unknown block (±42 nm, triangular). The value lies between -0.000042 and 0.000042 mm, and values near the centre 0 are more likely than values near the limits (triangular distribution). u = a/√6 = 0.000042/√6 = 1.71 × 10⁻⁵ mm.
7. **dL_r**: Repeatability (s = 16 nm from 11 earlier measurements, one reading). Mean of 1 readings, 0 mm. Instead of the scatter of these few readings, a standard deviation s_p = 0.000016 mm determined earlier from more data (ν = 10) is used, so u = s_p/√n = 1.60 × 10⁻⁵ mm.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 200 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 200 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 9.99994 mm
- standard uncertainty u = 0.0000407 mm
- expanded uncertainty U = 0.0000815 mm

Reference: UKAS M3003 ed. 6 K.6.10–K.6.11: u_c = 40.7 nm, U = 81.5 nm; 9.999 940 mm ± 0.081 µm.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## A thermocouple at 1000 °C (Intermediate)

An N-type thermocouple: furnace temperature and emf, with many small contributions.

<a id="m3003-k7-thermocouple-emf"></a>

### N-type thermocouple at 1000 °C: EMF with its full budget (UKAS M3003 K.7)

`m3003-k7-thermocouple-emf`

#### What is this project about?

Both budgets of K.7 in one model: the furnace temperature enters the EMF of the unknown thermocouple through its sensitivity 1/c_X = 38.5 µV/°C.


#### Step 1. What do we measure, and how is it calculated?

The result is **V_X** in µV (EMF of the N-type thermocouple at 1000 °C). It is not measured directly: it is calculated from the input quantities with the measurement model

```
V_X = V_iX + (1000 - t_X)/c_X + dV_iX1 + dV_iX2 + dV_LX + dt_TH/c_X + dV_R2 - dt_0X/c_X0 + dV_r
```

To keep the equation readable, 1 intermediate quantities are calculated first:

- `t_X = t_S + c_S*dV_iS1 + c_S*dV_iS2 + c_S*dV_R - c_S/c_S0*dt_0S + dt_D + dt_F + dt_r`


#### Step 2. The input quantities and their uncertainties

The model has 20 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **t_S**: Reference thermocouples, mean of the two (certificate U = 0.3 °C, k = 2). Best value 1000.5 °C. The source (e.g. a certificate) gives the expanded uncertainty U = 0.3 °C with k = 2, so the standard uncertainty is u = U/k = 0.3/2 = 0.150 °C (normal distribution).
2. **c_S**: Sensitivity of the R-type thermocouple at 1000 °C. An exact value, 0.077 °C/µV, with no uncertainty (for example a defined constant or a nominal value).
3. **c_S0**: Sensitivity of the R-type thermocouple at 0 °C. An exact value, 0.189 °C/µV, with no uncertainty (for example a defined constant or a nominal value).
4. **dV_iS1**: Microvoltmeter calibration (U = 2.0 µV, k = 2). Best value 0 µV. The source (e.g. a certificate) gives the expanded uncertainty U = 2 µV with k = 2, so the standard uncertainty is u = U/k = 2/2 = 1.00 µV (normal distribution).
5. **dV_iS2**: Microvoltmeter resolution (1 µV digit). Only limits are known: the value lies between -0.5 and 0.5 µV (0 ± 0.5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.5/√3 = 0.289 µV.
6. **dV_R**: Parasitic voltages of the switch contacts. Only limits are known: the value lies between -2 and 2 µV (0 ± 2), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 2/√3 = 1.15 µV.
7. **dt_0S**: Reference junction temperature of the standards. Only limits are known: the value lies between -0.1 and 0.1 °C (0 ± 0.1), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.1/√3 = 0.0577 °C.
8. **dt_D**: Drift of the reference thermocouples. Only limits are known: the value lies between -0.3 and 0.3 °C (0 ± 0.3), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.3/√3 = 0.173 °C.
9. **dt_F**: Non-uniformity of the furnace. Only limits are known: the value lies between -1 and 1 °C (0 ± 1), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 1/√3 = 0.577 °C.
10. **dt_r**: Repeatability of the furnace temperature (s = 0.10 °C, ν = 9). Mean of 1 readings, 0 °C. Instead of the scatter of these few readings, a standard deviation s_p = 0.1 °C determined earlier from more data (ν = 9) is used, so u = s_p/√n = 0.100 °C.
11. **V_iX**: Mean indication for the unknown thermocouple. An exact value, 36248 µV, with no uncertainty (for example a defined constant or a nominal value).
12. **c_X**: Sensitivity of the N-type thermocouple at 1000 °C. An exact value, 0.026 °C/µV, with no uncertainty (for example a defined constant or a nominal value).
13. **c_X0**: Sensitivity of the N-type thermocouple at 0 °C. An exact value, 0.038 °C/µV, with no uncertainty (for example a defined constant or a nominal value).
14. **dV_iX1**: Microvoltmeter calibration (U = 2.0 µV, k = 2). Best value 0 µV. The source (e.g. a certificate) gives the expanded uncertainty U = 2 µV with k = 2, so the standard uncertainty is u = U/k = 2/2 = 1.00 µV (normal distribution).
15. **dV_iX2**: Microvoltmeter resolution. Only limits are known: the value lies between -0.5 and 0.5 µV (0 ± 0.5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.5/√3 = 0.289 µV.
16. **dV_LX**: Compensating leads. Only limits are known: the value lies between -5 and 5 µV (0 ± 5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 5/√3 = 2.89 µV.
17. **dt_TH**: Inhomogeneity of the unknown thermocouple. Only limits are known: the value lies between -0.3 and 0.3 °C (0 ± 0.3), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.3/√3 = 0.173 °C.
18. **dV_R2**: Switch contacts (reading of the unknown). Only limits are known: the value lies between -2 and 2 µV (0 ± 2), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 2/√3 = 1.15 µV.
19. **dt_0X**: Reference junction temperature of the unknown. Only limits are known: the value lies between -0.1 and 0.1 °C (0 ± 0.1), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.1/√3 = 0.0577 °C.
20. **dV_r**: Repeatability of the voltage (s = 1.6 µV, ν = 9). Mean of 1 readings, 0 µV. Instead of the scatter of these few readings, a standard deviation s_p = 1.6 µV determined earlier from more data (ν = 9) is used, so u = s_p/√n = 1.60 µV.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 200 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 200 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 36230 µV
- standard uncertainty u = 25.9 µV
- expanded uncertainty U = 51.9 µV

Reference: UKAS M3003 ed. 6 K.7.20–K.7.21: u_c = 25.9 µV, U = 51.9 µV; 36 230 µV ± 52 µV (the estimate to the printed 10 µV).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="m3003-k7-thermocouple-furnace"></a>

### Temperature of the furnace in a thermocouple calibration at 1000 °C (UKAS M3003 K.7, first budget)

`m3003-k7-thermocouple-furnace`

#### What is this project about?

First part of K.7: the temperature at the hot junctions from the two reference thermocouples.


#### Step 1. What do we measure, and how is it calculated?

The result is **t_X** in °C (Temperature of the furnace at the thermocouples). It is not measured directly: it is calculated from the input quantities with the measurement model

```
t_X = t_S + c_S*dV_iS1 + c_S*dV_iS2 + c_S*dV_R - c_S/c_S0*dt_0S + dt_D + dt_F + dt_r
```


#### Step 2. The input quantities and their uncertainties

The model has 10 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **t_S**: Reference thermocouples, mean of the two (certificate U = 0.3 °C, k = 2). Best value 1000.5 °C. The source (e.g. a certificate) gives the expanded uncertainty U = 0.3 °C with k = 2, so the standard uncertainty is u = U/k = 0.3/2 = 0.150 °C (normal distribution).
2. **c_S**: Sensitivity of the R-type thermocouple at 1000 °C. An exact value, 0.077 °C/µV, with no uncertainty (for example a defined constant or a nominal value).
3. **c_S0**: Sensitivity of the R-type thermocouple at 0 °C. An exact value, 0.189 °C/µV, with no uncertainty (for example a defined constant or a nominal value).
4. **dV_iS1**: Microvoltmeter calibration (U = 2.0 µV, k = 2). Best value 0 µV. The source (e.g. a certificate) gives the expanded uncertainty U = 2 µV with k = 2, so the standard uncertainty is u = U/k = 2/2 = 1.00 µV (normal distribution).
5. **dV_iS2**: Microvoltmeter resolution (1 µV digit). Only limits are known: the value lies between -0.5 and 0.5 µV (0 ± 0.5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.5/√3 = 0.289 µV.
6. **dV_R**: Parasitic voltages of the switch contacts. Only limits are known: the value lies between -2 and 2 µV (0 ± 2), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 2/√3 = 1.15 µV.
7. **dt_0S**: Reference junction temperature of the standards. Only limits are known: the value lies between -0.1 and 0.1 °C (0 ± 0.1), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.1/√3 = 0.0577 °C.
8. **dt_D**: Drift of the reference thermocouples. Only limits are known: the value lies between -0.3 and 0.3 °C (0 ± 0.3), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.3/√3 = 0.173 °C.
9. **dt_F**: Non-uniformity of the furnace. Only limits are known: the value lies between -1 and 1 °C (0 ± 1), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 1/√3 = 0.577 °C.
10. **dt_r**: Repeatability of the furnace temperature (s = 0.10 °C, ν = 9). Mean of 1 readings, 0 °C. Instead of the scatter of these few readings, a standard deviation s_p = 0.1 °C determined earlier from more data (ν = 9) is used, so u = s_p/√n = 0.100 °C.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 200 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 200 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 1000.5 °C
- standard uncertainty u = 0.641 °C

Reference: UKAS M3003 ed. 6 K.7.19: u_c(t_X) = 0.641 °C.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Flow rate at standard conditions (Intermediate)

A volume flow rate converted to standard pressure and temperature: a product and ratio model.

<a id="m3003-k9-flowrate-standard-conditions"></a>

### Volume flowrate referred to standard pressure and temperature (UKAS M3003 K.9)

`m3003-k9-flowrate-standard-conditions`

#### What is this project about?

Flowrate from a 4–20 mA flowmeter at 1018 hPa and 295.1 K, expressed at 1013.25 hPa and 293.15 K; three partial budgets in one model.


#### Step 1. What do we measure, and how is it calculated?

The result is **Q_s** in L/s (Flowrate at standard conditions). It is not measured directly: it is calculated from the input quantities with the measurement model

```
Q_s = Q_m*(P_m/P_s)*(T_s/T_m) + dQ_s
```

To keep the equation readable, 3 intermediate quantities are calculated first:

- `Q_m = A*i + B + dq_cal - dq_d - dq_T`
- `P_m = p_m - dp_m - dp_f - dp_d - dp_T`
- `T_m = t_m - dt_m - dt_f - dt_d - dt_T`


#### Step 2. The input quantities and their uncertainties

The model has 19 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **A**: Slope of the calibration function. An exact value, 6.25 L s⁻¹/mA, with no uncertainty (for example a defined constant or a nominal value).
2. **B**: Intercept of the calibration function. An exact value, -24.95 L/s, with no uncertainty (for example a defined constant or a nominal value).
3. **i**: Measured current (U = 0.021 mA, k = 2). Best value 6 mA. The source (e.g. a certificate) gives the expanded uncertainty U = 0.021 mA with k = 2, so the standard uncertainty is u = U/k = 0.021/2 = 0.0105 mA (normal distribution).
4. **dq_cal**: Flowmeter calibration (U = 0.26 L/s, k = 2). Best value 0 L/s. The source (e.g. a certificate) gives the expanded uncertainty U = 0.26 L/s with k = 2, so the standard uncertainty is u = U/k = 0.26/2 = 0.130 L/s (normal distribution).
5. **dq_d**: Calibration drift. Only limits are known: the value lies between -0.23 and 0.23 L/s (0 ± 0.23), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.23/√3 = 0.133 L/s.
6. **dq_T**: Environmental effects. Only limits are known: the value lies between -0.11 and 0.11 L/s (0 ± 0.11), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.11/√3 = 0.0635 L/s.
7. **p_m**: Pressure indication (1 hPa resolution). Only limits are known: the value lies between 1017.5 and 1018.5 hPa (1018 ± 0.5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.5/√3 = 0.289 hPa.
8. **dp_m**: Gauge calibration (U = 1.8 hPa, k = 2). Best value 0 hPa. The source (e.g. a certificate) gives the expanded uncertainty U = 1.8 hPa with k = 2, so the standard uncertainty is u = U/k = 1.8/2 = 0.900 hPa (normal distribution).
9. **dp_f**: Uncorrected calibration error. Only limits are known: the value lies between -0.056 and 0.056 hPa (0 ± 0.056), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.056/√3 = 0.0323 hPa.
10. **dp_d**: Gauge drift. Only limits are known: the value lies between -1.25 and 1.25 hPa (0 ± 1.25), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 1.25/√3 = 0.722 hPa.
11. **dp_T**: Environmental effects on the gauge. Only limits are known: the value lies between -1 and 1 hPa (0 ± 1), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 1/√3 = 0.577 hPa.
12. **t_m**: Corrected temperature indication (0.1 K resolution). Only limits are known: the value lies between 295.05 and 295.15 K (295.1 ± 0.05), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.05/√3 = 0.0289 K.
13. **dt_m**: Thermometer calibration (U = 0.055 K, k = 2). Best value 0 K. The source (e.g. a certificate) gives the expanded uncertainty U = 0.055 K with k = 2, so the standard uncertainty is u = U/k = 0.055/2 = 0.0275 K (normal distribution).
14. **dt_f**: Residual error after the mean correction. Only limits are known: the value lies between -0.02 and 0.02 K (0 ± 0.02), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.02/√3 = 0.0115 K.
15. **dt_d**: Thermometer drift. Only limits are known: the value lies between -0.13 and 0.13 K (0 ± 0.13), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.13/√3 = 0.0751 K.
16. **dt_T**: Installation effects. Only limits are known: the value lies between -0.2 and 0.2 K (0 ± 0.2), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.2/√3 = 0.115 K.
17. **P_s**: Standard pressure. An exact value, 1013.25 hPa, with no uncertainty (for example a defined constant or a nominal value).
18. **T_s**: Standard temperature. An exact value, 293.15 K, with no uncertainty (for example a defined constant or a nominal value).
19. **dQ_s**: Repeatability of Q_s (s = 0.18 L/s, ν = 24, one reading). Mean of 1 readings, 0 L/s. Instead of the scatter of these few readings, a standard deviation s_p = 0.18 L/s determined earlier from more data (ν = 24) is used, so u = s_p/√n = 0.180 L/s.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 200 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 200 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 12.53 L/s
- standard uncertainty u = 0.275 L/s
- expanded uncertainty U = 0.55 L/s

Reference: UKAS M3003 ed. 6 K.9.10–K.9.12: u_c = 0.275 L/s, ν_eff = 130, U = 0.55 L/s; (12.53 ± 0.55) L/s.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## A PT100 probe (Intermediate)

First the probe is calibrated in a dry block; then it measures the temperature of a gas. The uncertainty of the calibration becomes an input of the measurement.

<a id="m3003-k11-gas-temperature"></a>

### Temperature of gas in a container from a calibrated PT100 probe (UKAS M3003 K.11, second budget)

`m3003-k11-gas-temperature`

#### What is this project about?

The probe L is attached to the outside of the vessel; its correction comes from the calibration function ΔT_u = 0.000 55·I_u + 0.419 °C.


#### Step 1. What do we measure, and how is it calculated?

The result is **T_g** in °C (Temperature of the gas). It is not measured directly: it is calculated from the input quantities with the measurement model

```
T_g = I_u - (0.00055*I_u + 0.419 + dT_cal + dT_fit + dT_drift) - dT_ug - dT_h - dT_r
```


#### Step 2. The input quantities and their uncertainties

The model has 7 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **I_u**: Indication of probe L (0.1 °C resolution). Only limits are known: the value lies between 23.350 and 23.45 °C (23.4 ± 0.05), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.05/√3 = 0.0289 °C.
2. **dT_cal**: Calibration of L (U = 0.084 49 °C, k = 2). Best value 0 °C. The source (e.g. a certificate) gives the expanded uncertainty U = 0.08449 °C with k = 2, so the standard uncertainty is u = U/k = 0.08449/2 = 0.0422 °C (normal distribution).
3. **dT_fit**: Linear fit of the calibration data of L. Only limits are known: the value lies between -0.012 and 0.012 °C (0 ± 0.012), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.012/√3 = 0.00693 °C.
4. **dT_drift**: Drift of L. Only limits are known: the value lies between -0.00467 and 0.00467 °C (0 ± 0.00467), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.00467/√3 = 0.00270 °C.
5. **dT_ug**: Difference between probe and gas. Only limits are known: the value lies between -0.1 and 0.1 °C (0 ± 0.1), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.1/√3 = 0.0577 °C.
6. **dT_h**: Self-heating of L on the vessel. Only limits are known: the value lies between -0.004 and 0.004 °C (0 ± 0.004), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.004/√3 = 0.00231 °C.
7. **dT_r**: Repeatability of the vessel temperature (s = 0.076 °C, ν = 49, mean of four). Mean of 4 readings, 0 °C. Instead of the scatter of these few readings, a standard deviation s_p = 0.076 °C determined earlier from more data (ν = 49) is used, so u = s_p/√n = 0.0380 °C.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 200 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 200 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 22.97 °C
- standard uncertainty u = 0.08638 °C
- expanded uncertainty U = 0.17276 °C

Reference: UKAS M3003 ed. 6 K.11.17–K.11.18: u_c = 0.086 38 °C, U = 0.172 76 °C; 22.97 °C ± 0.17 °C.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="m3003-k11-probe-calibration"></a>

### Calibration of a PT100 probe in a dry block against a reference probe (UKAS M3003 K.11, first budget)

`m3003-k11-probe-calibration`

#### What is this project about?

Error of the laboratory probe L at 20.4 °C against the reference probe S, whose calibration function is ΔT_S = 0.000 01·I_S + 0.0108 °C.


#### Step 1. What do we measure, and how is it calculated?

The result is **dT_L** in °C (Error of probe L). It is not measured directly: it is calculated from the input quantities with the measurement model

```
dT_L = I_L - (I_S - (0.00001*I_S + 0.0108 + dT_cal + dT_fit + dT_drift) - dT_SL) - d_rep
```


#### Step 2. The input quantities and their uncertainties

The model has 7 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **I_L**: Indication of probe L (0.1 °C resolution). Only limits are known: the value lies between 20.350 and 20.45 °C (20.4 ± 0.05), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.05/√3 = 0.0289 °C.
2. **I_S**: Indication of reference probe S (0.01 °C resolution; derived value). Only limits are known: the value lies between 19.974 and 19.984 °C (19.979 ± 0.005), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.005/√3 = 0.00289 °C.
3. **dT_cal**: Calibration of S (U = 0.024 °C, k = 2). Best value 0 °C. The source (e.g. a certificate) gives the expanded uncertainty U = 0.024 °C with k = 2, so the standard uncertainty is u = U/k = 0.024/2 = 0.0120 °C (normal distribution).
4. **dT_fit**: Linear fit of the calibration data of S. Only limits are known: the value lies between -0.005 and 0.005 °C (0 ± 0.005), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.005/√3 = 0.00289 °C.
5. **dT_drift**: Drift of S. Only limits are known: the value lies between -0.008 and 0.008 °C (0 ± 0.008), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.008/√3 = 0.00462 °C.
6. **dT_SL**: Gradients and installation in the block. Only limits are known: the value lies between -0.048 and 0.048 °C (0 ± 0.048), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.048/√3 = 0.0277 °C.
7. **d_rep**: Repeatability (s = 0.001 23 °C, ν = 24). Mean of 1 readings, 0 °C. Instead of the scatter of these few readings, a standard deviation s_p = 0.00123 °C determined earlier from more data (ν = 24) is used, so u = s_p/√n = 0.00123 °C.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 200 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 200 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- standard uncertainty u = 0.04225 °C
- expanded uncertainty U = 0.08449 °C

Reference: UKAS M3003 ed. 6 K.11.9: u_c = 0.042 25 °C, U = 0.084 49 °C.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Calibrating a laboratory balance (Intermediate)

A 220 g balance according to EURAMET cg-18: the error at several loads, with and without the air buoyancy correction, and the uncertainty of a weighing result in daily use.

<a id="cg18-h1-100g-option2"></a>

### Laboratory balance 220 g / 0.1 mg: error at 100 g with the buoyancy correction (EURAMET cg-18 H1, option 2)

`cg18-h1-100g-option2`

#### What is this project about?

The air density during calibration is known (990 hPa, 50 %RH, 21 °C: 1.173 kg/m³), so the buoyancy correction is applied; the test weights are 2 K from room temperature (convection).


#### Step 1. What do we measure, and how is it calculated?

The result is **E** in g (Error of indication). It is not measured directly: it is calculated from the input quantities with the measurement model

```
E = I_L + dI_dig0 + dI_rep + dI_digL + dI_ecc - (m_c + dm_D - m_N*(rho_a - rho_0)*(1/rho - 1/rho_c) + dm_conv)
```


#### Step 2. The input quantities and their uncertainties

The model has 13 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **I_L**: Indication at the load. An exact value, 100.0006 g, with no uncertainty (for example a defined constant or a nominal value).
2. **dI_dig0**: Rounding at zero. Only limits are known: the value lies between -0.00005 and 0.00005 g (0 ± 0.00005), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.00005/√3 = 2.89 × 10⁻⁵ g.
3. **dI_rep**: Repeatability (s from five readings, ν = 4, one reading). Mean of 1 readings, 0 g. Instead of the scatter of these few readings, a standard deviation s_p = 0.00011402 g determined earlier from more data (ν = 4) is used, so u = s_p/√n = 0.000114 g.
4. **dI_digL**: Rounding at the load. Only limits are known: the value lies between -0.00005 and 0.00005 g (0 ± 0.00005), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.00005/√3 = 2.89 × 10⁻⁵ g.
5. **dI_ecc**: Eccentricity. Only limits are known: the value lies between -0.0001 and 0.0001 g (0 ± 0.0001), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.0001/√3 = 5.77 × 10⁻⁵ g.
6. **m_c**: Conventional mass of the weights (certificate U/k). Best value 99.9999 g with standard uncertainty u = 0.000025 g (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
7. **dm_D**: Drift of the weights (D = 1.25·U). Only limits are known: the value lies between -0.0000625 and 0.0000625 g (0 ± 0.0000625), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.0000625/√3 = 3.61 × 10⁻⁵ g.
8. **m_N**: Nominal value of the load. An exact value, 100 g, with no uncertainty (for example a defined constant or a nominal value).
9. **rho_a**: Air density during calibration (CIPM formula; ΔT = 5 K by A3-2). Best value 1.173 kg/m³ with standard uncertainty u = 0.014 kg/m³ (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
10. **rho**: Density of the weights. Best value 7950 kg/m³ with standard uncertainty u = 70 kg/m³ (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
11. **rho_0**: Reference air density. An exact value, 1.2 kg/m³, with no uncertainty (for example a defined constant or a nominal value).
12. **rho_c**: Reference density of the weights. An exact value, 8000 kg/m³, with no uncertainty (for example a defined constant or a nominal value).
13. **dm_conv**: Convection, weights 2 K from room temperature (Table F2.1). Only limits are known: the value lies between -7.9674 × 10⁻⁵ and 7.9674 × 10⁻⁵ g (0 ± 7.9674 × 10⁻⁵), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 7.9674 × 10⁻⁵/√3 = 4.60 × 10⁻⁵ g.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It chooses the coverage factor k for a coverage probability of 95.5 %, using the effective degrees of freedom (Welch–Satterthwaite), and gives the **expanded uncertainty** U = k·u_c.

It also uses the **Monte Carlo method**: it draws 200 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 200 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- standard uncertainty u = 0.000149 g
- coverage factor k = 2.25
- expanded uncertainty U = 0.00033 g

Reference: EURAMET cg-18 v4.0 H1.3/A, option 2, 100 g: u(E) = 0.000149 g, ν_eff = 11, k = 2.25, U = 0.00033 g. The guide takes k from Table G.2 of the GUM (2.05 at ν = 62 is the entry for ν = 50); the exact t-value is 2.04.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="cg18-h1-220g-max-option1-delta-t"></a>

### Laboratory balance 220 g / 0.1 mg: error at 220 g with the buoyancy term for ΔT = 5 K (EURAMET cg-18 H1, option 1, formula 7.1.2-5e)

`cg18-h1-220g-max-option1-delta-t`

#### What is this project about?

As option 1, but the buoyancy uncertainty follows from the estimated temperature variation of the room, 5 K, instead of the worst case.


#### Step 1. What do we measure, and how is it calculated?

The result is **E** in g (Error of indication). It is not measured directly: it is calculated from the input quantities with the measurement model

```
E = I_L + dI_dig0 + dI_rep + dI_digL + dI_ecc - (m_c + dm_D + dm_B)
```


#### Step 2. The input quantities and their uncertainties

The model has 8 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **I_L**: Indication at the load. An exact value, 220.0014 g, with no uncertainty (for example a defined constant or a nominal value).
2. **dI_dig0**: Rounding at zero. Only limits are known: the value lies between -0.00005 and 0.00005 g (0 ± 0.00005), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.00005/√3 = 2.89 × 10⁻⁵ g.
3. **dI_rep**: Repeatability (s from five readings, ν = 4, one reading). Mean of 1 readings, 0 g. Instead of the scatter of these few readings, a standard deviation s_p = 0.00011402 g determined earlier from more data (ν = 4) is used, so u = s_p/√n = 0.000114 g.
4. **dI_digL**: Rounding at the load. Only limits are known: the value lies between -0.00005 and 0.00005 g (0 ± 0.00005), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.00005/√3 = 2.89 × 10⁻⁵ g.
5. **dI_ecc**: Eccentricity. Only limits are known: the value lies between -0.00022000 and 0.00022000 g (0 ± 0.00022000), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.00022000/√3 = 0.000127 g.
6. **m_c**: Conventional mass of the weights (certificate U/k). Best value 220.0001 g with standard uncertainty u = 0.000062 g (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
7. **dm_D**: Drift of the weights (D = 1.25·U). Only limits are known: the value lies between -0.000155 and 0.000155 g (0 ± 0.000155), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.000155/√3 = 8.95 × 10⁻⁵ g.
8. **dm_B**: Air buoyancy, not corrected, for ΔT = 5 K (7.1.2-5e). Best value 0 g with standard uncertainty u = 0.00044566 g (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It chooses the coverage factor k for a coverage probability of 95.5 %, using the effective degrees of freedom (Welch–Satterthwaite), and gives the **expanded uncertainty** U = k·u_c.

It also uses the **Monte Carlo method**: it draws 200 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 200 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 0.0013 g
- standard uncertainty u = 0.000491 g
- coverage factor k = 2
- expanded uncertainty U = 0.00098 g

Reference: EURAMET cg-18 v4.0 H1.3/A, option 1 with 7.1.2-5e, 220 g: u(E) = 0.000 491 g, ν_eff = 1377, k = 2.00, U = 0.000 98 g.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="cg18-h1-220g-max-option1"></a>

### Laboratory balance 220 g / 0.1 mg: error at 220 g without buoyancy correction (EURAMET cg-18 H1, option 1)

`cg18-h1-220g-max-option1`

#### What is this project about?

Not adjusted before calibration and no air density known: the buoyancy term is the conservative limit of formula 7.1.2-5d and dominates.


#### Step 1. What do we measure, and how is it calculated?

The result is **E** in g (Error of indication). It is not measured directly: it is calculated from the input quantities with the measurement model

```
E = I_L + dI_dig0 + dI_rep + dI_digL + dI_ecc - (m_c + dm_D + dm_B)
```


#### Step 2. The input quantities and their uncertainties

The model has 8 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **I_L**: Indication at the load. An exact value, 220.0014 g, with no uncertainty (for example a defined constant or a nominal value).
2. **dI_dig0**: Rounding at zero. Only limits are known: the value lies between -0.00005 and 0.00005 g (0 ± 0.00005), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.00005/√3 = 2.89 × 10⁻⁵ g.
3. **dI_rep**: Repeatability (s from five readings, ν = 4, one reading). Mean of 1 readings, 0 g. Instead of the scatter of these few readings, a standard deviation s_p = 0.00011402 g determined earlier from more data (ν = 4) is used, so u = s_p/√n = 0.000114 g.
4. **dI_digL**: Rounding at the load. Only limits are known: the value lies between -0.00005 and 0.00005 g (0 ± 0.00005), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.00005/√3 = 2.89 × 10⁻⁵ g.
5. **dI_ecc**: Eccentricity. Only limits are known: the value lies between -0.00022000 and 0.00022000 g (0 ± 0.00022000), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.00022000/√3 = 0.000127 g.
6. **m_c**: Conventional mass of the weights (certificate U/k). Best value 220.0001 g with standard uncertainty u = 0.000062 g (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
7. **dm_D**: Drift of the weights (D = 1.25·U). Only limits are known: the value lies between -0.000155 and 0.000155 g (0 ± 0.000155), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.000155/√3 = 8.95 × 10⁻⁵ g.
8. **dm_B**: Air buoyancy, not corrected (7.1.2-5d). Only limits are known: the value lies between -0.0033950 and 0.0033950 g (0 ± 0.0033950), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.0033950/√3 = 0.00196 g.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It chooses the coverage factor k for a coverage probability of 95.5 %, using the effective degrees of freedom (Welch–Satterthwaite), and gives the **expanded uncertainty** U = k·u_c.

It also uses the **Monte Carlo method**: it draws 200 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 200 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 0.0013 g
- standard uncertainty u = 0.001971 g
- coverage factor k = 2
- expanded uncertainty U = 0.00394 g

Reference: EURAMET cg-18 v4.0 H1.3/A, option 1 (7.1.2-5d), 220 g: u(E) = 0.001 971 g, ν_eff = 357 098, k = 2.00, U = 0.003 94 g.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="cg18-h1-220g-max-option2"></a>

### Laboratory balance 220 g / 0.1 mg: error at 220 g with the buoyancy correction (EURAMET cg-18 H1, option 2)

`cg18-h1-220g-max-option2`

#### What is this project about?

The air density during calibration is known (990 hPa, 50 %RH, 21 °C: 1.173 kg/m³), so the buoyancy correction is applied; the test weights are 2 K from room temperature (convection).


#### Step 1. What do we measure, and how is it calculated?

The result is **E** in g (Error of indication). It is not measured directly: it is calculated from the input quantities with the measurement model

```
E = I_L + dI_dig0 + dI_rep + dI_digL + dI_ecc - (m_c + dm_D - m_N*(rho_a - rho_0)*(1/rho - 1/rho_c) + dm_conv)
```


#### Step 2. The input quantities and their uncertainties

The model has 13 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **I_L**: Indication at the load. An exact value, 220.0014 g, with no uncertainty (for example a defined constant or a nominal value).
2. **dI_dig0**: Rounding at zero. Only limits are known: the value lies between -0.00005 and 0.00005 g (0 ± 0.00005), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.00005/√3 = 2.89 × 10⁻⁵ g.
3. **dI_rep**: Repeatability (s from five readings, ν = 4, one reading). Mean of 1 readings, 0 g. Instead of the scatter of these few readings, a standard deviation s_p = 0.00011402 g determined earlier from more data (ν = 4) is used, so u = s_p/√n = 0.000114 g.
4. **dI_digL**: Rounding at the load. Only limits are known: the value lies between -0.00005 and 0.00005 g (0 ± 0.00005), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.00005/√3 = 2.89 × 10⁻⁵ g.
5. **dI_ecc**: Eccentricity. Only limits are known: the value lies between -0.00022000 and 0.00022000 g (0 ± 0.00022000), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.00022000/√3 = 0.000127 g.
6. **m_c**: Conventional mass of the weights (certificate U/k). Best value 220.0001 g with standard uncertainty u = 0.000062 g (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
7. **dm_D**: Drift of the weights (D = 1.25·U). Only limits are known: the value lies between -0.000155 and 0.000155 g (0 ± 0.000155), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.000155/√3 = 8.95 × 10⁻⁵ g.
8. **m_N**: Nominal value of the load. An exact value, 220 g, with no uncertainty (for example a defined constant or a nominal value).
9. **rho_a**: Air density during calibration (CIPM formula; ΔT = 5 K by A3-2). Best value 1.173 kg/m³ with standard uncertainty u = 0.014 kg/m³ (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
10. **rho**: Density of the weights. Best value 7950 kg/m³ with standard uncertainty u = 70 kg/m³ (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
11. **rho_0**: Reference air density. An exact value, 1.2 kg/m³, with no uncertainty (for example a defined constant or a nominal value).
12. **rho_c**: Reference density of the weights. An exact value, 8000 kg/m³, with no uncertainty (for example a defined constant or a nominal value).
13. **dm_conv**: Convection, weights 2 K from room temperature (Table F2.1). Only limits are known: the value lies between -0.00015935 and 0.00015935 g (0 ± 0.00015935), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.00015935/√3 = 9.20 × 10⁻⁵ g.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It chooses the coverage factor k for a coverage probability of 95.5 %, using the effective degrees of freedom (Welch–Satterthwaite), and gives the **expanded uncertainty** U = k·u_c.

It also uses the **Monte Carlo method**: it draws 200 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 200 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- standard uncertainty u = 0.000226 g
- coverage factor k = 2.05
- expanded uncertainty U = 0.00046 g

Reference: EURAMET cg-18 v4.0 H1.3/A, option 2, 220 g: u(E) = 0.000226 g, ν_eff = 62, k = 2.05, U = 0.00046 g. The guide takes k from Table G.2 of the GUM (2.05 at ν = 62 is the entry for ν = 50); the exact t-value is 2.04.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="cg18-h1-220g-zero"></a>

### Laboratory balance 220 g / 0.1 mg: error at zero load (EURAMET cg-18 H1)

`cg18-h1-220g-zero`

#### What is this project about?

At zero load only the rounding at zero and the repeatability contribute; the repeatability with ν = 4 sets the coverage factor.


#### Step 1. What do we measure, and how is it calculated?

The result is **E** in g (Error of indication). It is not measured directly: it is calculated from the input quantities with the measurement model

```
E = I_L + dI_dig0 + dI_rep
```


#### Step 2. The input quantities and their uncertainties

The model has 3 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **I_L**: Indication at the load. An exact value, 0 g, with no uncertainty (for example a defined constant or a nominal value).
2. **dI_dig0**: Rounding at zero. Only limits are known: the value lies between -0.00005 and 0.00005 g (0 ± 0.00005), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.00005/√3 = 2.89 × 10⁻⁵ g.
3. **dI_rep**: Repeatability (s from five readings, ν = 4, one reading). Mean of 1 readings, 0 g. Instead of the scatter of these few readings, a standard deviation s_p = 0.00011402 g determined earlier from more data (ν = 4) is used, so u = s_p/√n = 0.000114 g.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It chooses the coverage factor k for a coverage probability of 95.5 %, using the effective degrees of freedom (Welch–Satterthwaite), and gives the **expanded uncertainty** U = k·u_c.

It also uses the **Monte Carlo method**: it draws 200 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 200 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 0 g
- standard uncertainty u = 0.000118 g
- coverage factor k = 2.87
- expanded uncertainty U = 0.00034 g

Reference: EURAMET cg-18 v4.0 H1.3/A, option 1, load 0: u(E) = 0.000 118 g, ν_eff = 4, k = 2.87, U = 0.000 34 g.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="cg18-h1-a-weighing-result"></a>

### Balance 220 g / 0.1 mg, not adjusted before calibration: uncertainty of a weighing result and minimum weight (EURAMET cg-18 H1.4/A)

`cg18-h1-a-weighing-result`

#### What is this project about?

Uncertainty of a weighing result in use, global uncertainty without correction of the readings and minimum weight from the calibration results and the assumed conditions of use.


#### Step 1. The idea: calibrating a balance

The Electronic balance, Max 220 g, d = 0.1 mg is tested with standard weights at 5 loads (EURAMET cg-18). At each load the error E = indication − reference mass and its uncertainty are found (repeatability, rounding of the display, eccentricity, the weights, air buoyancy).

A straight line (the error approximation function) is fitted through the errors, so that the error and its uncertainty are known for every load, not only the tested ones.

Then the **uncertainty of a weighing result in use** is calculated, including effects that occur in daily use (temperature, eccentricity, tare), and the **minimum weight**: the smallest load whose relative uncertainty is still acceptable.


#### Finally: what the source expects

Reference: EURAMET cg-18 v4.0 H1.1/A to H1.4/A: printed values of the uncertainty of a weighing result and the minimum weight.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="cg18-h1-b-weighing-result"></a>

### Balance 220 g / 0.1 mg, adjusted before calibration: uncertainty of a weighing result and minimum weight (EURAMET cg-18 H1.4/B)

`cg18-h1-b-weighing-result`

#### What is this project about?

Uncertainty of a weighing result in use, global uncertainty without correction of the readings and minimum weight from the calibration results and the assumed conditions of use.


#### Step 1. The idea: calibrating a balance

The Electronic balance, Max 220 g, d = 0.1 mg is tested with standard weights at 5 loads (EURAMET cg-18). At each load the error E = indication − reference mass and its uncertainty are found (repeatability, rounding of the display, eccentricity, the weights, air buoyancy).

A straight line (the error approximation function) is fitted through the errors, so that the error and its uncertainty are known for every load, not only the tested ones.

Then the **uncertainty of a weighing result in use** is calculated, including effects that occur in daily use (temperature, eccentricity, tare), and the **minimum weight**: the smallest load whose relative uncertainty is still acceptable.


#### Finally: what the source expects

Reference: EURAMET cg-18 v4.0 H1.1/B to H1.4/B: printed values of the uncertainty of a weighing result and the minimum weight; values affected by F75/F76 are the recomputed ones (see notes).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## A multi-interval balance (Intermediate)

A 60 kg scale with three weighing intervals. Each interval has its own resolution, which changes the budget along the load.

<a id="cg18-h2-60kg-10kg-option1"></a>

### Multi-interval balance 60 kg: error at 10 kg without buoyancy correction (EURAMET cg-18 H2, option 1)

`cg18-h2-60kg-10kg-option1`

#### What is this project about?

First load point, in the 2 g interval; no air density known and not adjusted before calibration: conservative buoyancy term 7.1.2-5d.


#### Step 1. What do we measure, and how is it calculated?

The result is **E** in g (Error of indication). It is not measured directly: it is calculated from the input quantities with the measurement model

```
E = I_L + dI_dig0 + dI_rep + dI_digL + dI_ecc - (m_N + dm_c + dm_D + dm_B)
```


#### Step 2. The input quantities and their uncertainties

The model has 9 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **I_L**: Indication at the load. An exact value, 10000 g, with no uncertainty (for example a defined constant or a nominal value).
2. **dI_dig0**: Rounding at zero. Only limits are known: the value lies between -1 and 1 g (0 ± 1), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 1/√3 = 0.577 g.
3. **dI_rep**: Repeatability (s = 1.095 g, ν = 4). Mean of 1 readings, 0 g. Instead of the scatter of these few readings, a standard deviation s_p = 1.0954 g determined earlier from more data (ν = 4) is used, so u = s_p/√n = 1.10 g.
4. **dI_digL**: Rounding at the load. Only limits are known: the value lies between -1 and 1 g (0 ± 1), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 1/√3 = 0.577 g.
5. **dI_ecc**: Eccentricity (7.1.1-10). Only limits are known: the value lies between -1.25 and 1.25 g (0 ± 1.25), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 1.25/√3 = 0.722 g.
6. **m_N**: Nominal value of the test load. An exact value, 10000 g, with no uncertainty (for example a defined constant or a nominal value).
7. **dm_c**: Weights at nominal value, F2 mpe = 0.16 g (7.1.2-3). Only limits are known: the value lies between -0.16 and 0.16 g (0 ± 0.16), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.16/√3 = 0.0924 g.
8. **dm_D**: Drift of the weights, D = mpe/2 (7.1.2-11). Only limits are known: the value lies between -0.08 and 0.08 g (0 ± 0.08), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.08/√3 = 0.0462 g.
9. **dm_B**: Air buoyancy, not adjusted before calibration (7.1.2-5d). Only limits are known: the value lies between -0.19000 and 0.19000 g (0 ± 0.19000), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.19000/√3 = 0.110 g.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It chooses the coverage factor k for a coverage probability of 95.5 %, using the effective degrees of freedom (Welch–Satterthwaite), and gives the **expanded uncertainty** U = k·u_c.

It also uses the **Monte Carlo method**: it draws 200 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 200 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 0 g
- standard uncertainty u = 1.552 g
- coverage factor k = 2.17
- expanded uncertainty U = 3.369 g

Reference: EURAMET cg-18 v4.0 H2.3/A, option 1 (7.1.2-5d), 10 kg: u(E) = 1.552 g, ν_eff = 16, k = 2.17, U = 3.369 g.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="cg18-h2-60kg-max-adjusted"></a>

### Multi-interval balance 60 kg, adjusted before calibration: error at Max (EURAMET cg-18 H2, second situation)

`cg18-h2-60kg-max-adjusted`

#### What is this project about?

Second situation: the instrument is adjusted with its built-in weights immediately before calibration; the repeatability is better (s = 2.236 g).


#### Step 1. What do we measure, and how is it calculated?

The result is **E** in g (Error of indication). It is not measured directly: it is calculated from the input quantities with the measurement model

```
E = I_L + dI_dig0 + dI_rep + dI_digL + dI_ecc - (m_N + dm_c + dm_D + dm_B)
```


#### Step 2. The input quantities and their uncertainties

The model has 9 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **I_L**: Indication at the load. An exact value, 60000 g, with no uncertainty (for example a defined constant or a nominal value).
2. **dI_dig0**: Rounding at zero. Only limits are known: the value lies between -1 and 1 g (0 ± 1), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 1/√3 = 0.577 g.
3. **dI_rep**: Repeatability (s = 2.236 g, ν = 4). Mean of 1 readings, 0 g. Instead of the scatter of these few readings, a standard deviation s_p = 2.2361 g determined earlier from more data (ν = 4) is used, so u = s_p/√n = 2.24 g.
4. **dI_digL**: Rounding at the load. Only limits are known: the value lies between -5 and 5 g (0 ± 5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 5/√3 = 2.89 g.
5. **dI_ecc**: Eccentricity (7.1.1-10). Only limits are known: the value lies between -7.5 and 7.5 g (0 ± 7.5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 7.5/√3 = 4.33 g.
6. **m_N**: Nominal value of the test load. An exact value, 60000 g, with no uncertainty (for example a defined constant or a nominal value).
7. **dm_c**: Weights at nominal value, F2 mpe = 0.90 g (7.1.2-3). Only limits are known: the value lies between -0.90000 and 0.90000 g (0 ± 0.90000), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.90000/√3 = 0.520 g.
8. **dm_D**: Drift of the weights, D = mpe/2 (7.1.2-11). Only limits are known: the value lies between -0.45000 and 0.45000 g (0 ± 0.45000), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.45000/√3 = 0.260 g.
9. **dm_B**: Air buoyancy, adjusted immediately before calibration (7.1.2-5c). Only limits are known: the value lies between -0.22500 and 0.22500 g (0 ± 0.22500), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.22500/√3 = 0.130 g.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It chooses the coverage factor k for a coverage probability of 95.5 %, using the effective degrees of freedom (Welch–Satterthwaite), and gives the **expanded uncertainty** U = k·u_c.

It also uses the **Monte Carlo method**: it draws 200 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 200 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 0 g
- standard uncertainty u = 5.739 g
- coverage factor k = 2.025
- expanded uncertainty U = 11.601 g

Reference: EURAMET cg-18 v4.0 H2.3/B, option 1 (7.1.2-5c), 60 kg: u(E) = 5.739 g, ν_eff = 172, k = 2.025, U = 11.601 g.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="cg18-h2-60kg-max-option1-delta-t"></a>

### Multi-interval balance 60 kg: error at Max with the buoyancy term for ΔT = 10 K (EURAMET cg-18 H2, option 1, formula 7.1.2-5e)

`cg18-h2-60kg-max-option1-delta-t`

#### What is this project about?

As option 1, with the buoyancy uncertainty from the expected temperature variation of the room (10 K, a room with windows).


#### Step 1. What do we measure, and how is it calculated?

The result is **E** in g (Error of indication). It is not measured directly: it is calculated from the input quantities with the measurement model

```
E = I_L + dI_dig0 + dI_rep + dI_digL + dI_ecc - (m_N + dm_c + dm_D + dm_B)
```


#### Step 2. The input quantities and their uncertainties

The model has 9 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **I_L**: Indication at the load. An exact value, 59990 g, with no uncertainty (for example a defined constant or a nominal value).
2. **dI_dig0**: Rounding at zero. Only limits are known: the value lies between -1 and 1 g (0 ± 1), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 1/√3 = 0.577 g.
3. **dI_rep**: Repeatability (s = 2.739 g, ν = 4). Mean of 1 readings, 0 g. Instead of the scatter of these few readings, a standard deviation s_p = 2.7386 g determined earlier from more data (ν = 4) is used, so u = s_p/√n = 2.74 g.
4. **dI_digL**: Rounding at the load. Only limits are known: the value lies between -5 and 5 g (0 ± 5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 5/√3 = 2.89 g.
5. **dI_ecc**: Eccentricity (7.1.1-10). Only limits are known: the value lies between -7.5 and 7.5 g (0 ± 7.5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 7.5/√3 = 4.33 g.
6. **m_N**: Nominal value of the test load. An exact value, 60000 g, with no uncertainty (for example a defined constant or a nominal value).
7. **dm_c**: Weights at nominal value, F2 mpe = 0.90 g (7.1.2-3). Only limits are known: the value lies between -0.90000 and 0.90000 g (0 ± 0.90000), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.90000/√3 = 0.520 g.
8. **dm_D**: Drift of the weights, D = mpe/2 (7.1.2-11). Only limits are known: the value lies between -0.45000 and 0.45000 g (0 ± 0.45000), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.45000/√3 = 0.260 g.
9. **dm_B**: Air buoyancy for ΔT = 10 K (7.1.2-5e). Best value 0 g with standard uncertainty u = 0.26933 g (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It chooses the coverage factor k for a coverage probability of 95.5 %, using the effective degrees of freedom (Welch–Satterthwaite), and gives the **expanded uncertainty** U = k·u_c.

It also uses the **Monte Carlo method**: it draws 200 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 200 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = -10 g
- standard uncertainty u = 5.948 g
- coverage factor k = 2.05
- expanded uncertainty U = 12.193 g

Reference: EURAMET cg-18 v4.0 H2.3/A, option 1 with 7.1.2-5e (ΔT = 10 K), 60 kg: u(E) = 5.948 g, ν_eff = 88, k = 2.05, U = 12.193 g.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="cg18-h2-60kg-max-option1"></a>

### Multi-interval balance 60 kg: error at Max without buoyancy correction (EURAMET cg-18 H2, option 1)

`cg18-h2-60kg-max-option1`

#### What is this project about?

Error at 60 kg in the 10 g interval, the largest U of the certificate.


#### Step 1. What do we measure, and how is it calculated?

The result is **E** in g (Error of indication). It is not measured directly: it is calculated from the input quantities with the measurement model

```
E = I_L + dI_dig0 + dI_rep + dI_digL + dI_ecc - (m_N + dm_c + dm_D + dm_B)
```


#### Step 2. The input quantities and their uncertainties

The model has 9 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **I_L**: Indication at the load. An exact value, 59990 g, with no uncertainty (for example a defined constant or a nominal value).
2. **dI_dig0**: Rounding at zero. Only limits are known: the value lies between -1 and 1 g (0 ± 1), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 1/√3 = 0.577 g.
3. **dI_rep**: Repeatability (s = 2.739 g, ν = 4). Mean of 1 readings, 0 g. Instead of the scatter of these few readings, a standard deviation s_p = 2.7386 g determined earlier from more data (ν = 4) is used, so u = s_p/√n = 2.74 g.
4. **dI_digL**: Rounding at the load. Only limits are known: the value lies between -5 and 5 g (0 ± 5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 5/√3 = 2.89 g.
5. **dI_ecc**: Eccentricity (7.1.1-10). Only limits are known: the value lies between -7.5 and 7.5 g (0 ± 7.5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 7.5/√3 = 4.33 g.
6. **m_N**: Nominal value of the test load. An exact value, 60000 g, with no uncertainty (for example a defined constant or a nominal value).
7. **dm_c**: Weights at nominal value, F2 mpe = 0.90 g (7.1.2-3). Only limits are known: the value lies between -0.90000 and 0.90000 g (0 ± 0.90000), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.90000/√3 = 0.520 g.
8. **dm_D**: Drift of the weights, D = mpe/2 (7.1.2-11). Only limits are known: the value lies between -0.45000 and 0.45000 g (0 ± 0.45000), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.45000/√3 = 0.260 g.
9. **dm_B**: Air buoyancy, not adjusted before calibration (7.1.2-5d). Only limits are known: the value lies between -1.125 and 1.125 g (0 ± 1.125), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 1.125/√3 = 0.650 g.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It chooses the coverage factor k for a coverage probability of 95.5 %, using the effective degrees of freedom (Welch–Satterthwaite), and gives the **expanded uncertainty** U = k·u_c.

It also uses the **Monte Carlo method**: it draws 200 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 200 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = -10 g
- standard uncertainty u = 5.978 g
- coverage factor k = 2.05
- expanded uncertainty U = 12.254 g

Reference: EURAMET cg-18 v4.0 H2.3/A, option 1 (7.1.2-5d), 60 kg: E = −10 g, u(E) = 5.978 g, ν_eff = 90, k = 2.05, U = 12.254 g; the tolerances cover the mpe of the test load and the k of Table G.2 (notes).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="cg18-h2-60kg-max-option2"></a>

### Multi-interval balance 60 kg: error at Max with the buoyancy correction (EURAMET cg-18 H2, option 2)

`cg18-h2-60kg-max-option2`

#### What is this project about?

The air density during calibration is computed from the CIPM formula and the buoyancy correction applied; the adjustment air density is unknown.


#### Step 1. What do we measure, and how is it calculated?

The result is **E** in g (Error of indication). It is not measured directly: it is calculated from the input quantities with the measurement model

```
E = I_L + dI_dig0 + dI_rep + dI_digL + dI_ecc - (m_N + dm_c + dm_D - m_N*(rho_a - rho_0)*(1/rho - 1/rho_c))
```


#### Step 2. The input quantities and their uncertainties

The model has 12 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **I_L**: Indication at the load. An exact value, 59990 g, with no uncertainty (for example a defined constant or a nominal value).
2. **dI_dig0**: Rounding at zero. Only limits are known: the value lies between -1 and 1 g (0 ± 1), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 1/√3 = 0.577 g.
3. **dI_rep**: Repeatability (s = 2.739 g, ν = 4). Mean of 1 readings, 0 g. Instead of the scatter of these few readings, a standard deviation s_p = 2.7386 g determined earlier from more data (ν = 4) is used, so u = s_p/√n = 2.74 g.
4. **dI_digL**: Rounding at the load. Only limits are known: the value lies between -5 and 5 g (0 ± 5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 5/√3 = 2.89 g.
5. **dI_ecc**: Eccentricity (7.1.1-10). Only limits are known: the value lies between -7.5 and 7.5 g (0 ± 7.5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 7.5/√3 = 4.33 g.
6. **m_N**: Nominal value of the test load. An exact value, 60000 g, with no uncertainty (for example a defined constant or a nominal value).
7. **dm_c**: Weights at nominal value, F2 mpe = 0.90 g (7.1.2-3). Only limits are known: the value lies between -0.90000 and 0.90000 g (0 ± 0.90000), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.90000/√3 = 0.520 g.
8. **dm_D**: Drift of the weights, D = mpe/2 (7.1.2-11). Only limits are known: the value lies between -0.45000 and 0.45000 g (0 ± 0.45000), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.45000/√3 = 0.260 g.
9. **rho_a**: Air density during calibration (CIPM formula; ΔT = 10 K by A3-2). Best value 1.173 kg/m³ with standard uncertainty u = 0.018 kg/m³ (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
10. **rho**: Density of the weights. Best value 7950 kg/m³ with standard uncertainty u = 70 kg/m³ (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
11. **rho_0**: Reference air density. An exact value, 1.2 kg/m³, with no uncertainty (for example a defined constant or a nominal value).
12. **rho_c**: Reference density of the weights. An exact value, 8000 kg/m³, with no uncertainty (for example a defined constant or a nominal value).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It chooses the coverage factor k for a coverage probability of 95.5 %, using the effective degrees of freedom (Welch–Satterthwaite), and gives the **expanded uncertainty** U = k·u_c.

It also uses the **Monte Carlo method**: it draws 200 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 200 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = -10 g
- standard uncertainty u = 5.941 g
- coverage factor k = 2.05
- expanded uncertainty U = 12.18 g

Reference: EURAMET cg-18 v4.0 H2.3/A, option 2, 60 kg: δm_B = 2.138·10⁻⁸·m_N, u(E) = 5.941 g, ν_eff = 88, k = 2.05, U = 12.180 g.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="cg18-h2-60kg-zero"></a>

### Multi-interval balance 60 kg: error at zero load (EURAMET cg-18 H2)

`cg18-h2-60kg-zero`

#### What is this project about?

A 60 kg multi-interval balance (2 g / 5 g / 10 g) calibrated as found, not adjusted before calibration.


#### Step 1. What do we measure, and how is it calculated?

The result is **E** in g (Error of indication). It is not measured directly: it is calculated from the input quantities with the measurement model

```
E = I_L + dI_dig0 + dI_rep
```


#### Step 2. The input quantities and their uncertainties

The model has 3 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **I_L**: Indication at the load. An exact value, 0 g, with no uncertainty (for example a defined constant or a nominal value).
2. **dI_dig0**: Rounding at zero. Only limits are known: the value lies between -1 and 1 g (0 ± 1), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 1/√3 = 0.577 g.
3. **dI_rep**: Repeatability (s = 1.095 g, ν = 4). Mean of 1 readings, 0 g. Instead of the scatter of these few readings, a standard deviation s_p = 1.0954 g determined earlier from more data (ν = 4) is used, so u = s_p/√n = 1.10 g.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It chooses the coverage factor k for a coverage probability of 95.5 %, using the effective degrees of freedom (Welch–Satterthwaite), and gives the **expanded uncertainty** U = k·u_c.

It also uses the **Monte Carlo method**: it draws 200 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 200 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 0 g
- standard uncertainty u = 1.238 g
- coverage factor k = 2.52
- expanded uncertainty U = 3.12 g

Reference: EURAMET cg-18 v4.0 H2.3/A, load 0: u(E) = 1.238 g, ν_eff = 6, k = 2.52, U = 3.120 g.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="cg18-h2-a-weighing-result"></a>

### Multi-interval scale 12/30/60 kg, not adjusted before calibration: uncertainty of a weighing result and minimum weight (EURAMET cg-18 H2.4/A)

`cg18-h2-a-weighing-result`

#### What is this project about?

Uncertainty of a weighing result in use, global uncertainty without correction of the readings and minimum weight from the calibration results and the assumed conditions of use.


#### Step 1. The idea: calibrating a balance

The Multi-interval scale, Max 12/30/60 kg, d = 2/5/10 g is tested with standard weights at 5 loads (EURAMET cg-18). At each load the error E = indication − reference mass and its uncertainty are found (repeatability, rounding of the display, eccentricity, the weights, air buoyancy).

A straight line (the error approximation function) is fitted through the errors, so that the error and its uncertainty are known for every load, not only the tested ones.

Then the **uncertainty of a weighing result in use** is calculated, including effects that occur in daily use (temperature, eccentricity, tare), and the **minimum weight**: the smallest load whose relative uncertainty is still acceptable.


#### Finally: what the source expects

Reference: EURAMET cg-18 v4.0 H2.1/A to H2.4/A: printed values of the uncertainty of a weighing result and the minimum weight; values affected by F75/F76 are the recomputed ones (see notes).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="cg18-h2-b-weighing-result"></a>

### Multi-interval scale 12/30/60 kg, adjusted before calibration: uncertainty of a weighing result and minimum weight (EURAMET cg-18 H2.4/B)

`cg18-h2-b-weighing-result`

#### What is this project about?

Uncertainty of a weighing result in use, global uncertainty without correction of the readings and minimum weight from the calibration results and the assumed conditions of use.


#### Step 1. The idea: calibrating a balance

The Multi-interval scale, Max 12/30/60 kg, d = 2/5/10 g is tested with standard weights at 5 loads (EURAMET cg-18). At each load the error E = indication − reference mass and its uncertainty are found (repeatability, rounding of the display, eccentricity, the weights, air buoyancy).

A straight line (the error approximation function) is fitted through the errors, so that the error and its uncertainty are known for every load, not only the tested ones.

Then the **uncertainty of a weighing result in use** is calculated, including effects that occur in daily use (temperature, eccentricity, tare), and the **minimum weight**: the smallest load whose relative uncertainty is still acceptable.


#### Finally: what the source expects

Reference: EURAMET cg-18 v4.0 H2.1/B to H2.4/B: printed values of the uncertainty of a weighing result and the minimum weight.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## A weighbridge (Intermediate)

A 30 t weighbridge tested with standard weights and substitution loads, as in real calibrations of large scales.

<a id="cg18-h3-30t-10t-weights"></a>

### Weighbridge 30 t: error at 10 t of standard weights (EURAMET cg-18 H3)

`cg18-h3-30t-10t-weights`

#### What is this project about?

The 10 × 1000 kg standard weights on the load receptor; this load m_ref is also the reference for the substitution loads.


#### Step 1. What do we measure, and how is it calculated?

The result is **E** in kg (Error of indication). It is not measured directly: it is calculated from the input quantities with the measurement model

```
E = I_L + dI_dig0 + dI_rep + dI_digL + dI_ecc + dI_time - (10 * (1000 + dm_c + dm_D + dm_B))
```


#### Step 2. The input quantities and their uncertainties

The model has 9 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **I_L**: Indication at the load. An exact value, 10010 kg, with no uncertainty (for example a defined constant or a nominal value).
2. **dI_dig0**: Rounding at zero. Only limits are known: the value lies between -0.5 and 0.5 kg (0 ± 0.5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.5/√3 = 0.289 kg.
3. **dI_rep**: Repeatability (s = 6.743 kg, ν = 5). Mean of 1 readings, 0 kg. Instead of the scatter of these few readings, a standard deviation s_p = 6.7429 kg determined earlier from more data (ν = 5) is used, so u = s_p/√n = 6.74 kg.
4. **dI_digL**: Rounding at the load. Only limits are known: the value lies between -0.5 and 0.5 kg (0 ± 0.5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.5/√3 = 0.289 kg.
5. **dI_ecc**: Eccentricity (7.1.1-10). Only limits are known: the value lies between -7.1977 and 7.1977 kg (0 ± 7.1977), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 7.1977/√3 = 4.16 kg.
6. **dI_time**: Creep and hysteresis (7.4.4-7). Only limits are known: the value lies between -1.3333 and 1.3333 kg (0 ± 1.3333), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 1.3333/√3 = 0.770 kg.
7. **dm_c**: Error of one 1000 kg weight (M1, at nominal value). Only limits are known: the value lies between -0.05 and 0.05 kg (0 ± 0.05), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.05/√3 = 0.0289 kg.
8. **dm_D**: Drift of one 1000 kg weight, D = mpe. Only limits are known: the value lies between -0.05 and 0.05 kg (0 ± 0.05), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.05/√3 = 0.0289 kg.
9. **dm_B**: Air buoyancy per 1000 kg (7.1.2-5d). Only limits are known: the value lies between -0.027500 and 0.027500 kg (0 ± 0.027500), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.027500/√3 = 0.0159 kg.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It chooses the coverage factor k for a coverage probability of 95.5 %, using the effective degrees of freedom (Welch–Satterthwaite), and gives the **expanded uncertainty** U = k·u_c.

It also uses the **Monte Carlo method**: it draws 200 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 200 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 10 kg
- standard uncertainty u = 7.98 kg
- coverage factor k = 2.32
- expanded uncertainty U = 19 kg

Reference: EURAMET cg-18 v4.0 H3.3/A, 10 000 kg of standard weights: E = 10 kg, u(E) = 7.98 kg, ν_eff = 9 (9.8 truncated), k = 2.32, U = 19 kg.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="cg18-h3-30t-15t-substitution"></a>

### Weighbridge 30 t: error at 15 t with a substitution load (EURAMET cg-18 H3)

`cg18-h3-30t-15t-substitution`

#### What is this project about?

5 t of standard weights on top of the substitution load L_sub1 (trailer and containers, adjusted to read like the 10 t of standard weights).


#### Step 1. What do we measure, and how is it calculated?

The result is **E** in kg (Error of indication). It is not measured directly: it is calculated from the input quantities with the measurement model

```
E = I_L + dI_dig0 + dI_rep + dI_digL + dI_ecc + dI_time - (5 * (1000 + dm_c + dm_D + dm_B) + 10 * (1000 + dm_c + dm_D + dm_B) + (dI_dig0_s1 + dI_rep_s1 + dI_digL_s1 + dI_ecc_s1 + dI_time_s1) - (dI_dig0_r1 + dI_rep_r1 + dI_digL_r1 + dI_ecc_r1 + dI_time_r1) + dm_Bsub)
```


#### Step 2. The input quantities and their uncertainties

The model has 20 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **I_L**: Indication at the load. An exact value, 15015 kg, with no uncertainty (for example a defined constant or a nominal value).
2. **dI_dig0**: Rounding at zero. Only limits are known: the value lies between -0.5 and 0.5 kg (0 ± 0.5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.5/√3 = 0.289 kg.
3. **dI_rep**: Repeatability (s = 6.743 kg, ν = 5). Mean of 1 readings, 0 kg. Instead of the scatter of these few readings, a standard deviation s_p = 6.7429 kg determined earlier from more data (ν = 5) is used, so u = s_p/√n = 6.74 kg.
4. **dI_digL**: Rounding at the load. Only limits are known: the value lies between -0.5 and 0.5 kg (0 ± 0.5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.5/√3 = 0.289 kg.
5. **dI_ecc**: Eccentricity (7.1.1-10). Only limits are known: the value lies between -10.797 and 10.797 kg (0 ± 10.797), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 10.797/√3 = 6.23 kg.
6. **dI_time**: Creep and hysteresis (7.4.4-7). Only limits are known: the value lies between -2 and 2 kg (0 ± 2), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 2/√3 = 1.15 kg.
7. **dm_c**: Error of one 1000 kg weight (M1, at nominal value). Only limits are known: the value lies between -0.05 and 0.05 kg (0 ± 0.05), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.05/√3 = 0.0289 kg.
8. **dm_D**: Drift of one 1000 kg weight, D = mpe. Only limits are known: the value lies between -0.05 and 0.05 kg (0 ± 0.05), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.05/√3 = 0.0289 kg.
9. **dm_B**: Air buoyancy per 1000 kg (7.1.2-5d). Only limits are known: the value lies between -0.027500 and 0.027500 kg (0 ± 0.027500), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.027500/√3 = 0.0159 kg.
10. **dI_dig0_s1**: Rounding at zero, I(L_sub1). Only limits are known: the value lies between -0.5 and 0.5 kg (0 ± 0.5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.5/√3 = 0.289 kg.
11. **dI_rep_s1**: Repeatability, I(L_sub1) (s = 6.743 kg, ν = 5). Mean of 1 readings, 0 kg. Instead of the scatter of these few readings, a standard deviation s_p = 6.7429 kg determined earlier from more data (ν = 5) is used, so u = s_p/√n = 6.74 kg.
12. **dI_digL_s1**: Rounding at the load, I(L_sub1). Only limits are known: the value lies between -0.5 and 0.5 kg (0 ± 0.5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.5/√3 = 0.289 kg.
13. **dI_ecc_s1**: Eccentricity, I(L_sub1) (7.1.1-10). Only limits are known: the value lies between -7.1977 and 7.1977 kg (0 ± 7.1977), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 7.1977/√3 = 4.16 kg.
14. **dI_time_s1**: Creep and hysteresis, I(L_sub1) (7.4.4-7). Only limits are known: the value lies between -1.3333 and 1.3333 kg (0 ± 1.3333), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 1.3333/√3 = 0.770 kg.
15. **dI_dig0_r1**: Rounding at zero, I(m_ref) at 10 t. Only limits are known: the value lies between -0.5 and 0.5 kg (0 ± 0.5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.5/√3 = 0.289 kg.
16. **dI_rep_r1**: Repeatability, I(m_ref) at 10 t (s = 6.743 kg, ν = 5). Mean of 1 readings, 0 kg. Instead of the scatter of these few readings, a standard deviation s_p = 6.7429 kg determined earlier from more data (ν = 5) is used, so u = s_p/√n = 6.74 kg.
17. **dI_digL_r1**: Rounding at the load, I(m_ref) at 10 t. Only limits are known: the value lies between -0.5 and 0.5 kg (0 ± 0.5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.5/√3 = 0.289 kg.
18. **dI_ecc_r1**: Eccentricity, I(m_ref) at 10 t (7.1.1-10). Only limits are known: the value lies between -7.1977 and 7.1977 kg (0 ± 7.1977), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 7.1977/√3 = 4.16 kg.
19. **dI_time_r1**: Creep and hysteresis, I(m_ref) at 10 t (7.4.4-7). Only limits are known: the value lies between -1.3333 and 1.3333 kg (0 ± 1.3333), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 1.3333/√3 = 0.770 kg.
20. **dm_Bsub**: Air buoyancy of the substitution loads (7.1.2-5d, ρ ≥ 4400 kg/m³). Only limits are known: the value lies between -0.27500 and 0.27500 kg (0 ± 0.27500), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.27500/√3 = 0.159 kg.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It chooses the coverage factor k for a coverage probability of 95.5 %, using the effective degrees of freedom (Welch–Satterthwaite), and gives the **expanded uncertainty** U = k·u_c.

It also uses the **Monte Carlo method**: it draws 200 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 200 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 15 kg
- standard uncertainty u = 14.6 kg

Reference: EURAMET cg-18 v4.0 H3.3/A, 15 000 kg = ½m_ref + L_sub1: E = 15 kg, u(E) = 14.60 kg (ν_eff = 109, k = 2.02, U = 29 kg; the coverage factor is not compared, see the notes).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="cg18-h3-30t-max-substitution"></a>

### Weighbridge 30 t: error at Max with two substitution loads (EURAMET cg-18 H3)

`cg18-h3-30t-max-substitution`

#### What is this project about?

10 t of standard weights on top of the substitution loads L_sub1 + L_sub2; the largest U of the certificate.


#### Step 1. What do we measure, and how is it calculated?

The result is **E** in kg (Error of indication). It is not measured directly: it is calculated from the input quantities with the measurement model

```
E = I_L + dI_dig0 + dI_rep + dI_digL + dI_ecc + dI_time - (10 * (1000 + dm_c + dm_D + dm_B) + 20 * (1000 + dm_c + dm_D + dm_B) + I_2 + (dI_dig0_s1 + dI_rep_s1 + dI_digL_s1 + dI_ecc_s1 + dI_time_s1) - (dI_dig0_r1 + dI_rep_r1 + dI_digL_r1 + dI_ecc_r1 + dI_time_r1) + (dI_dig0_s2 + dI_rep_s2 + dI_digL_s2 + dI_ecc_s2 + dI_time_s2) - (dI_dig0_r2 + dI_rep_r2 + dI_digL_r2 + dI_ecc_r2 + dI_time_r2) + dm_Bsub)
```


#### Step 2. The input quantities and their uncertainties

The model has 31 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **I_L**: Indication at the load. An exact value, 30040 kg, with no uncertainty (for example a defined constant or a nominal value).
2. **dI_dig0**: Rounding at zero. Only limits are known: the value lies between -0.5 and 0.5 kg (0 ± 0.5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.5/√3 = 0.289 kg.
3. **dI_rep**: Repeatability (s = 6.743 kg, ν = 5). Mean of 1 readings, 0 kg. Instead of the scatter of these few readings, a standard deviation s_p = 6.7429 kg determined earlier from more data (ν = 5) is used, so u = s_p/√n = 6.74 kg.
4. **dI_digL**: Rounding at the load. Only limits are known: the value lies between -0.5 and 0.5 kg (0 ± 0.5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.5/√3 = 0.289 kg.
5. **dI_ecc**: Eccentricity (7.1.1-10). Only limits are known: the value lies between -21.600 and 21.600 kg (0 ± 21.600), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 21.600/√3 = 12.5 kg.
6. **dI_time**: Creep and hysteresis (7.4.4-7). Only limits are known: the value lies between -4.0013 and 4.0013 kg (0 ± 4.0013), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 4.0013/√3 = 2.31 kg.
7. **dm_c**: Error of one 1000 kg weight (M1, at nominal value). Only limits are known: the value lies between -0.05 and 0.05 kg (0 ± 0.05), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.05/√3 = 0.0289 kg.
8. **dm_D**: Drift of one 1000 kg weight, D = mpe. Only limits are known: the value lies between -0.05 and 0.05 kg (0 ± 0.05), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.05/√3 = 0.0289 kg.
9. **dm_B**: Air buoyancy per 1000 kg (7.1.2-5d). Only limits are known: the value lies between -0.027500 and 0.027500 kg (0 ± 0.027500), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.027500/√3 = 0.0159 kg.
10. **dI_dig0_s1**: Rounding at zero, I(L_sub1). Only limits are known: the value lies between -0.5 and 0.5 kg (0 ± 0.5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.5/√3 = 0.289 kg.
11. **dI_rep_s1**: Repeatability, I(L_sub1) (s = 6.743 kg, ν = 5). Mean of 1 readings, 0 kg. Instead of the scatter of these few readings, a standard deviation s_p = 6.7429 kg determined earlier from more data (ν = 5) is used, so u = s_p/√n = 6.74 kg.
12. **dI_digL_s1**: Rounding at the load, I(L_sub1). Only limits are known: the value lies between -0.5 and 0.5 kg (0 ± 0.5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.5/√3 = 0.289 kg.
13. **dI_ecc_s1**: Eccentricity, I(L_sub1) (7.1.1-10). Only limits are known: the value lies between -7.1977 and 7.1977 kg (0 ± 7.1977), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 7.1977/√3 = 4.16 kg.
14. **dI_time_s1**: Creep and hysteresis, I(L_sub1) (7.4.4-7). Only limits are known: the value lies between -1.3333 and 1.3333 kg (0 ± 1.3333), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 1.3333/√3 = 0.770 kg.
15. **dI_dig0_r1**: Rounding at zero, I(m_ref) at 10 t. Only limits are known: the value lies between -0.5 and 0.5 kg (0 ± 0.5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.5/√3 = 0.289 kg.
16. **dI_rep_r1**: Repeatability, I(m_ref) at 10 t (s = 6.743 kg, ν = 5). Mean of 1 readings, 0 kg. Instead of the scatter of these few readings, a standard deviation s_p = 6.7429 kg determined earlier from more data (ν = 5) is used, so u = s_p/√n = 6.74 kg.
17. **dI_digL_r1**: Rounding at the load, I(m_ref) at 10 t. Only limits are known: the value lies between -0.5 and 0.5 kg (0 ± 0.5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.5/√3 = 0.289 kg.
18. **dI_ecc_r1**: Eccentricity, I(m_ref) at 10 t (7.1.1-10). Only limits are known: the value lies between -7.1977 and 7.1977 kg (0 ± 7.1977), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 7.1977/√3 = 4.16 kg.
19. **dI_time_r1**: Creep and hysteresis, I(m_ref) at 10 t (7.4.4-7). Only limits are known: the value lies between -1.3333 and 1.3333 kg (0 ± 1.3333), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 1.3333/√3 = 0.770 kg.
20. **I_2**: I₂ = I(L_sub1 + L_sub2) − I(m_ref + L_sub1) = 20 028 − 20 018 kg. An exact value, 10 kg, with no uncertainty (for example a defined constant or a nominal value).
21. **dI_dig0_s2**: Rounding at zero, I(L_sub1 + L_sub2). Only limits are known: the value lies between -0.5 and 0.5 kg (0 ± 0.5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.5/√3 = 0.289 kg.
22. **dI_rep_s2**: Repeatability, I(L_sub1 + L_sub2) (s = 6.743 kg, ν = 5). Mean of 1 readings, 0 kg. Instead of the scatter of these few readings, a standard deviation s_p = 6.7429 kg determined earlier from more data (ν = 5) is used, so u = s_p/√n = 6.74 kg.
23. **dI_digL_s2**: Rounding at the load, I(L_sub1 + L_sub2). Only limits are known: the value lies between -0.5 and 0.5 kg (0 ± 0.5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.5/√3 = 0.289 kg.
24. **dI_ecc_s2**: Eccentricity, I(L_sub1 + L_sub2) (7.1.1-10). Only limits are known: the value lies between -14.395 and 14.395 kg (0 ± 14.395), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 14.395/√3 = 8.31 kg.
25. **dI_time_s2**: Creep and hysteresis, I(L_sub1 + L_sub2) (7.4.4-7). Only limits are known: the value lies between -2.6667 and 2.6667 kg (0 ± 2.6667), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 2.6667/√3 = 1.54 kg.
26. **dI_dig0_r2**: Rounding at zero, I(m_ref + L_sub1) at 20 t. Only limits are known: the value lies between -0.5 and 0.5 kg (0 ± 0.5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.5/√3 = 0.289 kg.
27. **dI_rep_r2**: Repeatability, I(m_ref + L_sub1) at 20 t (s = 6.743 kg, ν = 5). Mean of 1 readings, 0 kg. Instead of the scatter of these few readings, a standard deviation s_p = 6.7429 kg determined earlier from more data (ν = 5) is used, so u = s_p/√n = 6.74 kg.
28. **dI_digL_r2**: Rounding at the load, I(m_ref + L_sub1) at 20 t. Only limits are known: the value lies between -0.5 and 0.5 kg (0 ± 0.5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.5/√3 = 0.289 kg.
29. **dI_ecc_r2**: Eccentricity, I(m_ref + L_sub1) at 20 t (7.1.1-10). Only limits are known: the value lies between -14.395 and 14.395 kg (0 ± 14.395), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 14.395/√3 = 8.31 kg.
30. **dI_time_r2**: Creep and hysteresis, I(m_ref + L_sub1) at 20 t (7.4.4-7). Only limits are known: the value lies between -2.6667 and 2.6667 kg (0 ± 2.6667), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 2.6667/√3 = 1.54 kg.
31. **dm_Bsub**: Air buoyancy of the substitution loads (7.1.2-5d, ρ ≥ 4400 kg/m³). Only limits are known: the value lies between -0.550275 and 0.550275 kg (0 ± 0.550275), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.550275/√3 = 0.318 kg.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It chooses the coverage factor k for a coverage probability of 95.5 %, using the effective degrees of freedom (Welch–Satterthwaite), and gives the **expanded uncertainty** U = k·u_c.

It also uses the **Monte Carlo method**: it draws 200 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 200 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 30 kg
- standard uncertainty u = 23.85 kg
- coverage factor k = 2
- expanded uncertainty U = 48 kg

Reference: EURAMET cg-18 v4.0 H3.3/A, 30 010 kg = m_ref + L_sub1 + L_sub2: E = 30 kg, u(E) = 23.85 kg, ν_eff = 783, k = 2.00, U = 48 kg.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="cg18-h3-30t-zero"></a>

### Weighbridge 30 t: error at zero load (EURAMET cg-18 H3)

`cg18-h3-30t-zero`

#### What is this project about?

A road-vehicle weighbridge 30 000 kg / 10 kg, outdoors, calibrated as found.


#### Step 1. What do we measure, and how is it calculated?

The result is **E** in kg (Error of indication). It is not measured directly: it is calculated from the input quantities with the measurement model

```
E = I_L + dI_dig0 + dI_rep
```


#### Step 2. The input quantities and their uncertainties

The model has 3 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **I_L**: Indication at the load. An exact value, 0 kg, with no uncertainty (for example a defined constant or a nominal value).
2. **dI_dig0**: Rounding at zero. Only limits are known: the value lies between -0.5 and 0.5 kg (0 ± 0.5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.5/√3 = 0.289 kg.
3. **dI_rep**: Repeatability (s = 6.743 kg, ν = 5). Mean of 1 readings, 0 kg. Instead of the scatter of these few readings, a standard deviation s_p = 6.7429 kg determined earlier from more data (ν = 5) is used, so u = s_p/√n = 6.74 kg.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It chooses the coverage factor k for a coverage probability of 95.5 %, using the effective degrees of freedom (Welch–Satterthwaite), and gives the **expanded uncertainty** U = k·u_c.

It also uses the **Monte Carlo method**: it draws 200 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 200 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 0 kg
- standard uncertainty u = 6.75 kg
- coverage factor k = 2.65
- expanded uncertainty U = 18 kg

Reference: EURAMET cg-18 v4.0 H3.3/A, load 0: u(E) = 6.75 kg, ν_eff = 5, k = 2.65, U = 18 kg.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="cg18-h3-a-weighing-result"></a>

### Weighbridge 30 t / 10 kg, not adjusted before calibration: uncertainty of a weighing result and minimum weight (EURAMET cg-18 H3.4/A)

`cg18-h3-a-weighing-result`

#### What is this project about?

Uncertainty of a weighing result in use, global uncertainty without correction of the readings and minimum weight from the calibration results and the assumed conditions of use.


#### Step 1. The idea: calibrating a balance

The Electronic weighbridge, Max 30 000 kg, d = 10 kg (calibrated with d_T = 1 kg and substitution loads) is tested with standard weights at 7 loads (EURAMET cg-18). At each load the error E = indication − reference mass and its uncertainty are found (repeatability, rounding of the display, eccentricity, the weights, air buoyancy).

A straight line (the error approximation function) is fitted through the errors, so that the error and its uncertainty are known for every load, not only the tested ones.

Then the **uncertainty of a weighing result in use** is calculated, including effects that occur in daily use (temperature, eccentricity, tare), and the **minimum weight**: the smallest load whose relative uncertainty is still acceptable.


#### Finally: what the source expects

Reference: EURAMET cg-18 v4.0 H3.1/A to H3.4/A: printed values of the uncertainty of a weighing result and the minimum weight; values affected by F75/F76 are the recomputed ones (see notes).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="cg18-h3-b-weighing-result"></a>

### Weighbridge 30 t / 10 kg, adjusted before calibration: uncertainty of a weighing result and minimum weight (EURAMET cg-18 H3.4/B)

`cg18-h3-b-weighing-result`

#### What is this project about?

Uncertainty of a weighing result in use, global uncertainty without correction of the readings and minimum weight from the calibration results and the assumed conditions of use.


#### Step 1. The idea: calibrating a balance

The Electronic weighbridge, Max 30 000 kg, d = 10 kg (calibrated with d_T = 1 kg and substitution loads) is tested with standard weights at 7 loads (EURAMET cg-18). At each load the error E = indication − reference mass and its uncertainty are found (repeatability, rounding of the display, eccentricity, the weights, air buoyancy).

A straight line (the error approximation function) is fitted through the errors, so that the error and its uncertainty are known for every load, not only the tested ones.

Then the **uncertainty of a weighing result in use** is calculated, including effects that occur in daily use (temperature, eccentricity, tare), and the **minimum weight**: the smallest load whose relative uncertainty is still acceptable.


#### Finally: what the source expects

Reference: EURAMET cg-18 v4.0 H3.1/B to H3.4/B: printed values of the uncertainty of a weighing result and the minimum weight; values affected by F75/F76 are the recomputed ones (see notes).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## A falling-ball viscometer (Intermediate)

Viscosity from the fall time of a ball. A model with ratios of densities and times.

<a id="nist-tn1900-e3-falling-ball-viscometer"></a>

### Falling-ball viscometer (NIST TN 1900 E3)

`nist-tn1900-e3-falling-ball-viscometer`

#### What is this project about?

TN 1900 E3: dynamic viscosity of a NaOH solution by comparison with a calibration liquid; large relative uncertainties of the travel times make the output skewed.


#### Step 1. What do we measure, and how is it calculated?

The result is **mu_M** in mPa s (Dynamic viscosity of the NaOH solution). It is not measured directly: it is calculated from the input quantities with the measurement model

```
mu_M = mu_C * (rho_B - rho_M) / (rho_B - rho_C) * t_M / t_C
```


#### Step 2. The input quantities and their uncertainties

The model has 6 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **mu_C**: Viscosity of the calibration liquid. Best value 4.63 mPa s with standard uncertainty u = 0.0463 mPa s (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **rho_B**: Density of the ball. Best value 2217 kg/m³ with standard uncertainty u = 0.5 kg/m³ (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **rho_C**: Density of the calibration liquid. Best value 810 kg/m³ with standard uncertainty u = 0.5 kg/m³ (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **rho_M**: Density of the NaOH solution. Best value 1180 kg/m³ with standard uncertainty u = 0.5 kg/m³ (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
5. **t_C**: Travel time of the ball in the calibration liquid. Best value 36.6 s with standard uncertainty u = 5.49 s (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
6. **t_M**: Travel time of the ball in the NaOH solution. Best value 61 s with standard uncertainty u = 6.1 s (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.

It also uses the **Monte Carlo method**: it draws 2 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 2 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 5.69 mPa s
- standard uncertainty u = 1.0269 mPa s

Reference: NIST TN 1900 E3: estimate 5.69 mPa s from the measurement equation; Monte Carlo 5.82 mPa s, u = 1.11 mPa s, 95 % interval [4.05, 8.39] mPa s. GUM u = 1.027 mPa s: first-order law of propagation evaluated analytically (see note).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Airspeed with a Pitot tube (Intermediate)

The speed follows from a pressure difference through a square root. This example also contains a published inconsistency between the equation and the printed result (see the notes).

<a id="nist-tn1900-e4-pitot-tube"></a>

### Airspeed with a Pitot tube (NIST TN 1900 E4)

`nist-tn1900-e4-pitot-tube`

#### What is this project about?

TN 1900 E4 (after Kline and McClintock, 1953): airspeed from the differential pressure, the air pressure and the temperature.


#### Step 1. What do we measure, and how is it calculated?

The result is **v** in m/s (Airspeed). It is not measured directly: it is calculated from the input quantities with the measurement model

```
v = sqrt(2 * 1000 * Delta * R_s * T / (1000 * p))
```


#### Step 2. The input quantities and their uncertainties

The model has 4 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **Delta**: Difference between total and static pressure. Best value 1.993 kPa with standard uncertainty u = 0.0125 kPa (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **R_s**: Specific gas constant of dry air. An exact value, 287.058 J/(kg K), with no uncertainty (for example a defined constant or a nominal value).
3. **T**: Air temperature. Best value 292.8 K with standard uncertainty u = 0.055 K (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **p**: Air pressure. Best value 101.4 kPa with standard uncertainty u = 1.05 kPa (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 57.474 m/s
- standard uncertainty u = 0.34790 m/s

Reference: NIST TN 1900 E4: v = 40.64 m/s, u(v) = 0.25 m/s (relative 0.61 %), Monte Carlo 95 % interval [40.17, 41.13] m/s, each multiplied by √2 to correct for the missing factor 2 (see note).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Molecular weight of CO₂ (Intermediate)

From the atomic weights of carbon and oxygen, which are given as intervals.

<a id="nist-tn1900-e8-molecular-weight-co2"></a>

### Molecular weight of carbon dioxide (NIST TN 1900 E8)

`nist-tn1900-e8-molecular-weight-co2`

#### What is this project about?

TN 1900 E8: M_r(CO₂) = A_r(C) + 2A_r(O) from the IUPAC interval atomic weights; the output distribution is trapezoidal and the GUM interval is too wide.


#### Step 1. What do we measure, and how is it calculated?

The result is **M_r** (Relative molecular mass of CO₂). It is not measured directly: it is calculated from the input quantities with the measurement model

```
M_r = A_C + 2 * A_O
```


#### Step 2. The input quantities and their uncertainties

The model has 2 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **A_C**: Standard atomic weight of carbon. Only limits are known: the value lies between 12.0096 and 12.0116 (12.0106 ± 0.0010000), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.0010000/√3 = 0.000577.
2. **A_O**: Standard atomic weight of oxygen. Only limits are known: the value lies between 15.99903 and 15.99977 (15.9994 ± 0.00037000), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.00037000/√3 = 0.000214.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 44.0094
- standard uncertainty u = 0.0007

Reference: NIST TN 1900 E8: M_r(CO₂) = 44.0094, u = 0.0007, exact shortest 95 % interval [44.0080, 44.0108], whose unrounded end points 44.008045 and 44.010755 follow from the trapezoidal distribution (see note); tolerance ≈ 5 × the Monte Carlo dispersion at M = 10⁶.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## A cadmium standard by Monte Carlo (Intermediate)

NIST's version of the Eurachem cadmium example, evaluated with the Monte Carlo method.

<a id="nist-tn1900-e9-cadmium-standard"></a>

### Cadmium calibration standard, Monte Carlo (NIST TN 1900 E9)

`nist-tn1900-e9-cadmium-standard`

#### What is this project about?

TN 1900 E9: the QUAM A1 cadmium standard c = 1000 m P / V re-evaluated with the Monte Carlo method, with a rectangular purity and a triangular volume.


#### Step 1. What do we measure, and how is it calculated?

The result is **c_Cd** in mg/L (Cadmium concentration of the standard). It is not measured directly: it is calculated from the input quantities with the measurement model

```
c_Cd = 1000 * m * P / V
```


#### Step 2. The input quantities and their uncertainties

The model has 3 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **m**: Mass of cadmium. Best value 100.28 mg with standard uncertainty u = 0.05 mg (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **P**: Purity of the metal (mass fraction). Only limits are known: the value lies between 0.9998 and 1 (0.9999 ± 0.00010000), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.00010000/√3 = 5.77 × 10⁻⁵.
3. **V**: Volume of the solution. The value lies between 99.828536 and 100.171464 mL, and values near the centre 100 are more likely than values near the limits (triangular distribution). u = a/√6 = 0.171464/√6 = 0.0700 mL.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 1002.7 mg/L
- standard uncertainty u = 0.9 mg/L

Reference: NIST TN 1900 E9: c = 1002.7 mg/L, u ≈ 0.9 mg/L, Monte Carlo 95 % interval [1001.0, 1004.4] mg/L.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## U-shaped inputs (Intermediate)

A microwave step attenuator whose mismatch terms are U-shaped; GUM and Monte Carlo are compared.

<a id="nist-tn1900-e11-step-attenuator"></a>

### Microwave step attenuator, U-shaped inputs (NIST TN 1900 E11)

`nist-tn1900-e11-step-attenuator`

#### What is this project about?

TN 1900 E11: EA-4/02 S7 re-evaluated with the Monte Carlo method; the dominant arcsine mismatch term makes the distribution of a linear sum bimodal.


#### Step 1. What do we measure, and how is it calculated?

The result is **L_X** in dB (Attenuation of the 30 dB step). It is not measured directly: it is calculated from the input quantities with the measurement model

```
L_X = L_S + dL_S + dL_D + dL_M + dL_K + dL_ib - dL_ia + dL_0b - dL_0a
```


#### Step 2. The input quantities and their uncertainties

The model has 9 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **L_S**: Mean of four attenuation differences. Best value 30.0402 dB with standard uncertainty u = 0.00525 dB. Because it is based on few data (ν = 3 degrees of freedom), a Student t distribution is used: it has heavier tails than the normal distribution and makes the coverage factor larger.
2. **dL_S**: Correction from the calibration of the reference attenuator. Only limits are known: the value lies between -0.0013301 and 0.0073301 dB (0.003 ± 0.0043301), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.0043301/√3 = 0.00250 dB.
3. **dL_D**: Drift of the reference attenuator. The value swings between -0.0019799 and 0.0019799 dB and is most often near the limits (U-shaped or arcsine distribution; e.g. a temperature cycling around a set point, or a mismatch phase). u = a/√2 = 0.0019799/√2 = 0.00140 dB.
4. **dL_M**: Mismatch loss correction. The value swings between -0.0282843 and 0.0282843 dB and is most often near the limits (U-shaped or arcsine distribution; e.g. a temperature cycling around a set point, or a mismatch phase). u = a/√2 = 0.0282843/√2 = 0.0200 dB.
5. **dL_K**: Leakage correction. The value swings between -0.003 and 0.003 dB and is most often near the limits (U-shaped or arcsine distribution; e.g. a temperature cycling around a set point, or a mismatch phase). u = a/√2 = 0.003/√2 = 0.00212 dB.
6. **dL_ia**: Resolution of the reference detector at 0 dB. Only limits are known: the value lies between -0.0005196 and 0.0005196 dB (0 ± 0.0005196), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.0005196/√3 = 0.000300 dB.
7. **dL_ib**: Resolution of the reference detector at 30 dB. Only limits are known: the value lies between -0.0005196 and 0.0005196 dB (0 ± 0.0005196), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.0005196/√3 = 0.000300 dB.
8. **dL_0a**: Resolution of the null detector at 0 dB. Best value 0 dB with standard uncertainty u = 0.002 dB (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
9. **dL_0b**: Resolution of the null detector at 30 dB. Best value 0 dB with standard uncertainty u = 0.002 dB (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.

It also uses the **Monte Carlo method**: it draws 2 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 2 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

Reference: NIST TN 1900 E11: L_X = 30.043 dB, u = 0.0224 dB, 95 % interval [30.006, 30.081] dB (Monte Carlo, M = 10⁷).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Normal or t inputs, 95 % or 99 % (Intermediate)

The thermal expansion coefficient evaluated four times: with normal or Student t inputs, and for 95 % or 99 % coverage. See how both choices change the interval.

<a id="nist-tn1900-e13-thermal-expansion-gaussian-95"></a>

### Thermal expansion coefficient, Gaussian inputs, 95 % (NIST TN 1900 E13)

`nist-tn1900-e13-thermal-expansion-gaussian-95`

#### What is this project about?

TN 1900 E13: linear thermal expansion coefficient of a copper bar from two length measurements at two temperatures; gaussian inputs, 95 % coverage.


#### Step 1. What do we measure, and how is it calculated?

The result is **alpha** in 10⁻⁶ K⁻¹ (Linear thermal expansion coefficient of the copper bar). It is not measured directly: it is calculated from the input quantities with the measurement model

```
alpha = 1e6 * (L_1 - L_0) / (L_0 * (T_1 - T_0))
```


#### Step 2. The input quantities and their uncertainties

The model has 4 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **L_0**: Length at T₀. Best value 1.4999 m with standard uncertainty u = 0.0001 m (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **L_1**: Length at T₁. Best value 1.5021 m with standard uncertainty u = 0.0002 m (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **T_0**: Initial temperature. Best value 288.15 K with standard uncertainty u = 0.02 K (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **T_1**: Final temperature. Best value 373.1 K with standard uncertainty u = 0.05 K (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It chooses the coverage factor k for a coverage probability of 95.0 %, using the effective degrees of freedom (Welch–Satterthwaite), and gives the **expanded uncertainty** U = k·u_c.

It also uses the **Monte Carlo method**: it draws 4 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 4 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 17.27 10⁻⁶ K⁻¹
- standard uncertainty u = 1.8 10⁻⁶ K⁻¹

Reference: NIST TN 1900 E13 (Gaussian inputs): α = 1.73 × 10⁻⁵ K⁻¹, u = 0.18 × 10⁻⁵ K⁻¹, 95 % interval (1.38, 2.07) × 10⁻⁵ K⁻¹ (Monte Carlo, M = 10⁷). Tolerances: half a unit of the printed digit plus the Monte Carlo dispersion.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="nist-tn1900-e13-thermal-expansion-gaussian-99"></a>

### Thermal expansion coefficient, Gaussian inputs, 99 % (NIST TN 1900 E13)

`nist-tn1900-e13-thermal-expansion-gaussian-99`

#### What is this project about?

TN 1900 E13: linear thermal expansion coefficient of a copper bar from two length measurements at two temperatures; gaussian inputs, 99 % coverage.


#### Step 1. What do we measure, and how is it calculated?

The result is **alpha** in 10⁻⁶ K⁻¹ (Linear thermal expansion coefficient of the copper bar). It is not measured directly: it is calculated from the input quantities with the measurement model

```
alpha = 1e6 * (L_1 - L_0) / (L_0 * (T_1 - T_0))
```


#### Step 2. The input quantities and their uncertainties

The model has 4 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **L_0**: Length at T₀. Best value 1.4999 m with standard uncertainty u = 0.0001 m (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **L_1**: Length at T₁. Best value 1.5021 m with standard uncertainty u = 0.0002 m (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **T_0**: Initial temperature. Best value 288.15 K with standard uncertainty u = 0.02 K (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **T_1**: Final temperature. Best value 373.1 K with standard uncertainty u = 0.05 K (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It chooses the coverage factor k for a coverage probability of 99.0 %, using the effective degrees of freedom (Welch–Satterthwaite), and gives the **expanded uncertainty** U = k·u_c.

It also uses the **Monte Carlo method**: it draws 4 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 4 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 17.27 10⁻⁶ K⁻¹
- standard uncertainty u = 1.8 10⁻⁶ K⁻¹

Reference: NIST TN 1900 E13 (Gaussian inputs): α = 1.73 × 10⁻⁵ K⁻¹, u = 0.18 × 10⁻⁵ K⁻¹, 99 % interval (1.27, 2.18) × 10⁻⁵ K⁻¹ (Monte Carlo, M = 10⁷). Tolerances: half a unit of the printed digit plus the Monte Carlo dispersion.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="nist-tn1900-e13-thermal-expansion-student-95"></a>

### Thermal expansion coefficient, Student t₃ inputs, 95 % (NIST TN 1900 E13)

`nist-tn1900-e13-thermal-expansion-student-95`

#### What is this project about?

TN 1900 E13: linear thermal expansion coefficient of a copper bar from two length measurements at two temperatures; student t₃ inputs, 95 % coverage.


#### Step 1. What do we measure, and how is it calculated?

The result is **alpha** in 10⁻⁶ K⁻¹ (Linear thermal expansion coefficient of the copper bar). It is not measured directly: it is calculated from the input quantities with the measurement model

```
alpha = 1e6 * (L_1 - L_0) / (L_0 * (T_1 - T_0))
```


#### Step 2. The input quantities and their uncertainties

The model has 4 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **L_0**: Length at T₀. Best value 1.4999 m with standard uncertainty u = 5.77 × 10⁻⁵ m. Because it is based on few data (ν = 3 degrees of freedom), a Student t distribution is used: it has heavier tails than the normal distribution and makes the coverage factor larger.
2. **L_1**: Length at T₁. Best value 1.5021 m with standard uncertainty u = 0.000115 m. Because it is based on few data (ν = 3 degrees of freedom), a Student t distribution is used: it has heavier tails than the normal distribution and makes the coverage factor larger.
3. **T_0**: Initial temperature. Best value 288.15 K with standard uncertainty u = 0.0115 K. Because it is based on few data (ν = 3 degrees of freedom), a Student t distribution is used: it has heavier tails than the normal distribution and makes the coverage factor larger.
4. **T_1**: Final temperature. Best value 373.1 K with standard uncertainty u = 0.0289 K. Because it is based on few data (ν = 3 degrees of freedom), a Student t distribution is used: it has heavier tails than the normal distribution and makes the coverage factor larger.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.

It also uses the **Monte Carlo method**: it draws 4 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 4 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

Reference: NIST TN 1900 E13 (Student inputs): α = 1.73 × 10⁻⁵ K⁻¹, u = 0.18 × 10⁻⁵ K⁻¹, 95 % interval (1.40, 2.05) × 10⁻⁵ K⁻¹ (Monte Carlo, M = 10⁷). Tolerances: half a unit of the printed digit plus the Monte Carlo dispersion; the sample standard deviation of t₃ inputs converges slowly (infinite fourth moment).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="nist-tn1900-e13-thermal-expansion-student-99"></a>

### Thermal expansion coefficient, Student t₃ inputs, 99 % (NIST TN 1900 E13)

`nist-tn1900-e13-thermal-expansion-student-99`

#### What is this project about?

TN 1900 E13: linear thermal expansion coefficient of a copper bar from two length measurements at two temperatures; student t₃ inputs, 99 % coverage.


#### Step 1. What do we measure, and how is it calculated?

The result is **alpha** in 10⁻⁶ K⁻¹ (Linear thermal expansion coefficient of the copper bar). It is not measured directly: it is calculated from the input quantities with the measurement model

```
alpha = 1e6 * (L_1 - L_0) / (L_0 * (T_1 - T_0))
```


#### Step 2. The input quantities and their uncertainties

The model has 4 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **L_0**: Length at T₀. Best value 1.4999 m with standard uncertainty u = 5.77 × 10⁻⁵ m. Because it is based on few data (ν = 3 degrees of freedom), a Student t distribution is used: it has heavier tails than the normal distribution and makes the coverage factor larger.
2. **L_1**: Length at T₁. Best value 1.5021 m with standard uncertainty u = 0.000115 m. Because it is based on few data (ν = 3 degrees of freedom), a Student t distribution is used: it has heavier tails than the normal distribution and makes the coverage factor larger.
3. **T_0**: Initial temperature. Best value 288.15 K with standard uncertainty u = 0.0115 K. Because it is based on few data (ν = 3 degrees of freedom), a Student t distribution is used: it has heavier tails than the normal distribution and makes the coverage factor larger.
4. **T_1**: Final temperature. Best value 373.1 K with standard uncertainty u = 0.0289 K. Because it is based on few data (ν = 3 degrees of freedom), a Student t distribution is used: it has heavier tails than the normal distribution and makes the coverage factor larger.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.

It also uses the **Monte Carlo method**: it draws 4 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 4 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

Reference: NIST TN 1900 E13 (Student inputs): α = 1.73 × 10⁻⁵ K⁻¹, u = 0.18 × 10⁻⁵ K⁻¹, 99 % interval (1.15, 2.30) × 10⁻⁵ K⁻¹ (Monte Carlo, M = 10⁷). Tolerances: half a unit of the printed digit plus the Monte Carlo dispersion; the sample standard deviation of t₃ inputs converges slowly (infinite fourth moment).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Mass calibration: when the first order is not enough (Intermediate)

The mass calibration of JCGM 101 with the first-order GUM, with second-order terms and with Monte Carlo. A small nonlinearity makes the first-order uncertainty too small.

<a id="jcgm101-9.3-mass-calibration-higher-order"></a>

### Mass calibration (GUM with second-order terms)

`jcgm101-9.3-mass-calibration-higher-order`

#### What is this project about?

JCGM 101 9.3: the same model evaluated with the higher-order terms of GUM 5.1.2.


#### Step 1. What do we measure, and how is it calculated?

The result is **dm** in mg (Deviation of the conventional mass of W from 100 g). It is not measured directly: it is calculated from the input quantities with the measurement model

```
dm = (m_Rc + dm_Rc) * (1 + (rho_a - 1.2) * (1/rho_W - 1/rho_R)) - 100000
```


#### Step 2. The input quantities and their uncertainties

The model has 5 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **m_Rc**: Conventional mass of the reference weight. Best value 100000 mg with standard uncertainty u = 0.05 mg (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **dm_Rc**: Mass of the small weight added to balance. Best value 1.234 mg with standard uncertainty u = 0.02 mg (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **rho_a**: Air density. Only limits are known: the value lies between 1.1000 and 1.3 kg/m³ (1.2 ± 0.1), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.1/√3 = 0.0577 kg/m³.
4. **rho_W**: Density of the weight W. Only limits are known: the value lies between 7000 and 9000 kg/m³ (8000 ± 1000), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 1000/√3 = 577 kg/m³.
5. **rho_R**: Density of the reference weight R. Only limits are known: the value lies between 7950 and 8050 kg/m³ (8000 ± 50), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 50/√3 = 28.9 kg/m³.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.
6. Because the model is clearly nonlinear, second-order terms are added (GUM 5.1.2 and its 2026 amendment).

It also uses the **Monte Carlo method**: it draws 2 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 2 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 1.234 mg
- standard uncertainty u = 0.075 mg

Reference: JCGM 101:2008 Table 6 (GUF2) and the independent MCM reference.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="jcgm101-9.3-mass-calibration"></a>

### Mass calibration (first-order GUM)

`jcgm101-9.3-mass-calibration`

#### What is this project about?

JCGM 101 9.3: calibration of a 100 g weight by comparison with a reference weight; first-order GUM vs MCM.


#### Step 1. What do we measure, and how is it calculated?

The result is **dm** in mg (Deviation of the conventional mass of W from 100 g). It is not measured directly: it is calculated from the input quantities with the measurement model

```
dm = (m_Rc + dm_Rc) * (1 + (rho_a - 1.2) * (1/rho_W - 1/rho_R)) - 100000
```


#### Step 2. The input quantities and their uncertainties

The model has 5 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **m_Rc**: Conventional mass of the reference weight. Best value 100000 mg with standard uncertainty u = 0.05 mg (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **dm_Rc**: Mass of the small weight added to balance. Best value 1.234 mg with standard uncertainty u = 0.02 mg (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **rho_a**: Air density. Only limits are known: the value lies between 1.1000 and 1.3 kg/m³ (1.2 ± 0.1), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.1/√3 = 0.0577 kg/m³.
4. **rho_W**: Density of the weight W. Only limits are known: the value lies between 7000 and 9000 kg/m³ (8000 ± 1000), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 1000/√3 = 577 kg/m³.
5. **rho_R**: Density of the reference weight R. Only limits are known: the value lies between 7950 and 8050 kg/m³ (8000 ± 50), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 50/√3 = 28.9 kg/m³.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.

It also uses the **Monte Carlo method**: it draws 2 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 2 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 1.234 mg
- standard uncertainty u = 0.0539 mg

Reference: GUM: JCGM 101:2008 Table 6. MCM: independent 2 × 10⁸-trial reference (validation/reference_values.json); tolerances ≈ 5 × engine dispersion at M = 2 × 10⁶.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Two outputs from the same inputs (Intermediate)

Two results calculated from shared inputs are correlated. JCGM 102 gives their covariance matrix and a coverage region (an ellipse) instead of two separate intervals.

<a id="jcgm102-9.2.2-additive-gaussian"></a>

### Additive bivariate model, Gaussian inputs (JCGM 102 9.2.2)

`jcgm102-9.2.2-additive-gaussian`

#### What is this project about?

JCGM 102:2011 9.2.2: two outputs sharing a common Gaussian effect; the GUM results are exact and validated by the Monte Carlo method.


#### Step 1. What do we measure, and how is it calculated?

This project has 2 results (output quantities) that are calculated together from the same inputs. Each has its own equation:

- **Y1**: `Y1 = X1 + X3`
- **Y2**: `Y2 = X2 + X3`


#### Step 2. The input quantities and their uncertainties

The model has 3 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **X1**. Best value 0 with standard uncertainty u = 1 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **X2**. Best value 0 with standard uncertainty u = 1 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **X3**. Best value 0 with standard uncertainty u = 1 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It chooses the coverage factor k for a coverage probability of 95.0 %, using the effective degrees of freedom (Welch–Satterthwaite), and gives the **expanded uncertainty** U = k·u_c.

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

Reference: JCGM 102:2011 Table 3 (GUF and MCM with M = 10⁶).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="jcgm102-9.2.3-additive-rectangular"></a>

### Additive bivariate model, rectangular common effect (JCGM 102 9.2.3)

`jcgm102-9.2.3-additive-rectangular`

#### What is this project about?

JCGM 102:2011 9.2.3: as 9.2.2 with the common effect X₃ rectangular (standard deviation 1); the GUM coverage factors are not validated.


#### Step 1. What do we measure, and how is it calculated?

This project has 2 results (output quantities) that are calculated together from the same inputs. Each has its own equation:

- **Y1**: `Y1 = X1 + X3`
- **Y2**: `Y2 = X2 + X3`


#### Step 2. The input quantities and their uncertainties

The model has 3 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **X1**. Best value 0 with standard uncertainty u = 1 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **X2**. Best value 0 with standard uncertainty u = 1 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **X3**. Only limits are known: the value lies between -1.7321 and 1.7321 (0 ± 1.7321), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 1.7321/√3 = 1.00.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It chooses the coverage factor k for a coverage probability of 95.0 %, using the effective degrees of freedom (Welch–Satterthwaite), and gives the **expanded uncertainty** U = k·u_c.

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

Reference: JCGM 102:2011 Table 4 (GUF and MCM with M = 10⁶).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="jcgm102-9.2.4-additive-dominant-rectangular"></a>

### Additive bivariate model, dominant rectangular common effect (JCGM 102 9.2.4)

`jcgm102-9.2.4-additive-dominant-rectangular`

#### What is this project about?

JCGM 102:2011 9.2.4: the common rectangular effect has standard deviation 3; r(y₁, y₂) = 0.9 and the GUM k_p and k_q are about 7 % and 20 % too large.


#### Step 1. What do we measure, and how is it calculated?

This project has 2 results (output quantities) that are calculated together from the same inputs. Each has its own equation:

- **Y1**: `Y1 = X1 + X3`
- **Y2**: `Y2 = X2 + X3`


#### Step 2. The input quantities and their uncertainties

The model has 3 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **X1**. Best value 0 with standard uncertainty u = 1 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **X2**. Best value 0 with standard uncertainty u = 1 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **X3**. Only limits are known: the value lies between -5.1962 and 5.1962 (0 ± 5.1962), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 5.1962/√3 = 3.00.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It chooses the coverage factor k for a coverage probability of 95.0 %, using the effective degrees of freedom (Welch–Satterthwaite), and gives the **expanded uncertainty** U = k·u_c.

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

Reference: JCGM 102:2011 Table 5 (GUF and MCM with M = 10⁶).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## pH by interpolation (Intermediate)

The pH of a solution from two buffer standards, at 25 °C and at another temperature. The variants show what happens when the certified values of the standards are correlated.

<a id="gum5-e1-ph-temperature-interpolation-correlated"></a>

### pH at 23.7 °C by three linear interpolations, certified values of each standard correlated across temperatures (JCGM GUM-5 Example 1)

`gum5-e1-ph-temperature-interpolation-correlated`

#### What is this project about?

As the independent case, but the pH values of each standard at 20 °C and 25 °C are fully correlated, as the shared Bates–Guggenheim convention suggests (GUM-5 1.10.3.3).


#### Step 1. What do we measure, and how is it calculated?

The result is **pH_X** (pH of the test solution at T_X). It is not measured directly: it is calculated from the input quantities with the measurement model

```
pH_X = pH_T1 + (pH_T2 - pH_T1)*(T_X - T1)/(T2 - T1)
```

To keep the equation readable, 2 intermediate quantities are calculated first:

- `pH_T1 = pH_S1_T1 + (pH_S2_T1 - pH_S1_T1)*(E_X - E_S1)/(E_S2 - E_S1)`
- `pH_T2 = pH_S1_T2 + (pH_S2_T2 - pH_S1_T2)*(E_X - E_S1)/(E_S2 - E_S1)`


#### Step 2. The input quantities and their uncertainties

The model has 10 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **T_X**: Temperature of the test solution. Best value 23.7 °C with standard uncertainty u = 0.0058 °C (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **E_X**: Potential in the test solution. Best value -1.875 mV with standard uncertainty u = 0.025 mV (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **E_S1**: Potential in standard S1. Best value 6.15 mV with standard uncertainty u = 0.029 mV (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **E_S2**: Potential in standard S2. Best value -26.35 mV with standard uncertainty u = 0.029 mV (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
5. **pH_S1_T1**: pH of S1 at 25 °C. Best value 6.864 with standard uncertainty u = 0.0051 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
6. **pH_S2_T1**: pH of S2 at 25 °C. Best value 7.4157 with standard uncertainty u = 0.0051 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
7. **pH_S1_T2**: pH of S1 at 20 °C. Best value 6.8796 with standard uncertainty u = 0.0051 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
8. **pH_S2_T2**: pH of S2 at 20 °C. Best value 7.4323 with standard uncertainty u = 0.0051 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
9. **T1**: Certificate temperature T₁. An exact value, 25 °C, with no uncertainty (for example a defined constant or a nominal value).
10. **T2**: Certificate temperature T₂. An exact value, 20 °C, with no uncertainty (for example a defined constant or a nominal value).

**Correlated inputs.** Some inputs are not independent, because a common effect (the same standard, the same instrument, the same temperature) acts on both. The correlation coefficient r tells how strongly; the program adds the corresponding term to the combined uncertainty:

- pH_S1_T1 ↔ pH_S1_T2: r = 1
- pH_S2_T1 ↔ pH_S2_T2: r = 1


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 7.0043
- standard uncertainty u = 0.0041

Reference: JCGM GUM-5:2026, 1.9.1: pH_X = 7.0043 (see note), u = 0.0041.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="gum5-e1-ph-temperature-interpolation"></a>

### pH at 23.7 °C by three linear interpolations, independent certified values (JCGM GUM-5 Example 1)

`gum5-e1-ph-temperature-interpolation`

#### What is this project about?

The test solution is at 23.7 °C, between the certificate temperatures 20 °C and 25 °C: interpolation in potential at each temperature, then in temperature; all inputs independent (GUM-5 1.7.1).


#### Step 1. What do we measure, and how is it calculated?

The result is **pH_X** (pH of the test solution at T_X). It is not measured directly: it is calculated from the input quantities with the measurement model

```
pH_X = pH_T1 + (pH_T2 - pH_T1)*(T_X - T1)/(T2 - T1)
```

To keep the equation readable, 2 intermediate quantities are calculated first:

- `pH_T1 = pH_S1_T1 + (pH_S2_T1 - pH_S1_T1)*(E_X - E_S1)/(E_S2 - E_S1)`
- `pH_T2 = pH_S1_T2 + (pH_S2_T2 - pH_S1_T2)*(E_X - E_S1)/(E_S2 - E_S1)`


#### Step 2. The input quantities and their uncertainties

The model has 10 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **T_X**: Temperature of the test solution. Best value 23.7 °C with standard uncertainty u = 0.0058 °C (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **E_X**: Potential in the test solution. Best value -1.875 mV with standard uncertainty u = 0.025 mV (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **E_S1**: Potential in standard S1. Best value 6.15 mV with standard uncertainty u = 0.029 mV (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **E_S2**: Potential in standard S2. Best value -26.35 mV with standard uncertainty u = 0.029 mV (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
5. **pH_S1_T1**: pH of S1 at 25 °C. Best value 6.864 with standard uncertainty u = 0.0051 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
6. **pH_S2_T1**: pH of S2 at 25 °C. Best value 7.4157 with standard uncertainty u = 0.0051 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
7. **pH_S1_T2**: pH of S1 at 20 °C. Best value 6.8796 with standard uncertainty u = 0.0051 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
8. **pH_S2_T2**: pH of S2 at 20 °C. Best value 7.4323 with standard uncertainty u = 0.0051 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
9. **T1**: Certificate temperature T₁. An exact value, 25 °C, with no uncertainty (for example a defined constant or a nominal value).
10. **T2**: Certificate temperature T₂. An exact value, 20 °C, with no uncertainty (for example a defined constant or a nominal value).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 7.0043
- standard uncertainty u = 0.0032

Reference: JCGM GUM-5:2026, 1.9.1: pH_X,T1 = 7.0002, pH_X,T2 = 7.0161, pH_X = 7.0043; u recomputed for independent inputs (0.0032); the guide prints 0.0041 (see note).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="gum5-e1-ph-two-point-25c-correlated-standards"></a>

### pH by two-point interpolation with fully correlated pH standards (JCGM GUM-5 Example 1)

`gum5-e1-ph-two-point-25c-correlated-standards`

#### What is this project about?

As the 25 °C case, but with correlation 1 between the certified pH values, which share the Bates–Guggenheim convention.


#### Step 1. What do we measure, and how is it calculated?

The result is **pH_X** (pH of the test solution). It is not measured directly: it is calculated from the input quantities with the measurement model

```
pH_X = pH_S1 + (pH_S2 - pH_S1)*(E_X - E_S1)/(E_S2 - E_S1)
```


#### Step 2. The input quantities and their uncertainties

The model has 5 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **E_X**: Potential in the test solution (mean of 4). Best value -1.875 mV with standard uncertainty u = 0.025 mV (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **E_S1**: Potential in standard S1 (SRM 186-I-g). Best value 6.15 mV with standard uncertainty u = 0.0289 mV (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **E_S2**: Potential in standard S2 (SRM 186-II-g). Best value -26.35 mV with standard uncertainty u = 0.0289 mV (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **pH_S1**: Certified pH of S1 at 25 °C. Best value 6.864 with standard uncertainty u = 0.0051 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
5. **pH_S2**: Certified pH of S2 at 25 °C. Best value 7.4157 with standard uncertainty u = 0.0051 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).

**Correlated inputs.** Some inputs are not independent, because a common effect (the same standard, the same instrument, the same temperature) acts on both. The correlation coefficient r tells how strongly; the program adds the corresponding term to the combined uncertainty:

- pH_S1 ↔ pH_S2: r = 1


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 7.0002
- standard uncertainty u = 0.0051

Reference: JCGM GUM-5:2026, 1.10.3.5: u(pH_X) = 0.0051 with perfectly correlated standards.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="gum5-e1-ph-two-point-25c"></a>

### pH of a test solution by two-point interpolation at 25 °C (JCGM GUM-5 Example 1)

`gum5-e1-ph-two-point-25c`

#### What is this project about?

Bracketing between two NIST phosphate pH standards measured with the same cell; inputs independent, as in IUPAC practice.


#### Step 1. What do we measure, and how is it calculated?

The result is **pH_X** (pH of the test solution). It is not measured directly: it is calculated from the input quantities with the measurement model

```
pH_X = pH_S1 + (pH_S2 - pH_S1)*(E_X - E_S1)/(E_S2 - E_S1)
```


#### Step 2. The input quantities and their uncertainties

The model has 5 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **E_X**: Potential in the test solution (mean of 4). Best value -1.875 mV with standard uncertainty u = 0.025 mV (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **E_S1**: Potential in standard S1 (SRM 186-I-g). Best value 6.15 mV with standard uncertainty u = 0.0289 mV (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **E_S2**: Potential in standard S2 (SRM 186-II-g). Best value -26.35 mV with standard uncertainty u = 0.0289 mV (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **pH_S1**: Certified pH of S1 at 25 °C. Best value 6.864 with standard uncertainty u = 0.0051 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
5. **pH_S2**: Certified pH of S2 at 25 °C. Best value 7.4157 with standard uncertainty u = 0.0051 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 7.0002
- standard uncertainty u = 0.0041

Reference: JCGM GUM-5:2026, 1.8.1: pH_X = 7.0002, u = 0.0041.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## A greenhouse gas inventory (Intermediate)

Emissions from agricultural machinery. When the same activity data are used for several gases, the results are correlated and the total uncertainty changes.

<a id="gum5-e5-ghg-inventory-agriculture-shared-activity"></a>

### Greenhouse gas inventory, mobile machinery in agriculture (UK 2018): activity data shared by CH₄ and N₂O (JCGM GUM-5 Example 5)

`gum5-e5-ghg-inventory-agriculture-shared-activity`

#### What is this project about?

CO₂-equivalent emissions E = Σ s_i A_i F_i (IPCC category A4ci) and their standard uncertainty by the LPU, with A₂ = A₃ and A₅ = A₆ as one quantity each (5.4.9).


#### Step 1. What do we measure, and how is it calculated?

The result is **E** in kt (Total CO₂-equivalent emissions of the sub-sector). It is not measured directly: it is calculated from the input quantities with the measurement model

```
E = 11/3*A1*F1 + 25*A2*F2 + 298*A2*F3 + 11/3*A4*F4 + 25*A5*F5 + 298*A5*F6
```


#### Step 2. The input quantities and their uncertainties

The model has 10 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **A1**: Activity data, gas oil (CO2); U_rel = 38.6 % (95 %). Best value 56317.092 TJ with standard uncertainty u = 11091 TJ (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **F1**: Emission factor, CO2, gas oil; U_rel = 2.7 % (95 %). Best value 0.020438 kt/TJ with standard uncertainty u = 0.00028154 kt/TJ (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **A2**: Activity data, gas oil (CH₄ and N₂O); U_rel = 1.6 % (95 %). Best value 56317.092 TJ with standard uncertainty u = 459.73 TJ (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **F2**: Emission factor, CH4, gas oil; U_rel = 80.0 % (95 %). Best value 3.5368 × 10⁻⁶ kt/TJ with standard uncertainty u = 1.4436 × 10⁻⁶ kt/TJ (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
5. **F3**: Emission factor, N2O, gas oil; U_rel = 216.3 % (95 %). Best value 3.0984 × 10⁻⁶ kt/TJ with standard uncertainty u = 3.4193 × 10⁻⁶ kt/TJ (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
6. **A4**: Activity data, petrol (CO2); U_rel = 50.7 % (95 %). Best value 67.1856 TJ with standard uncertainty u = 17.379 TJ (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
7. **F4**: Emission factor, CO2, petrol; U_rel = 2.0 % (95 %). Best value 0.019127 kt/TJ with standard uncertainty u = 0.00019517 kt/TJ (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
8. **A5**: Activity data, petrol (CH₄ and N₂O); U_rel = 1.6 % (95 %). Best value 67.1856 TJ with standard uncertainty u = 0.54845 TJ (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
9. **F5**: Emission factor, CH4, petrol; U_rel = 80.0 % (95 %). Best value 4.8654 × 10⁻⁵ kt/TJ with standard uncertainty u = 1.9859 × 10⁻⁵ kt/TJ (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
10. **F6**: Emission factor, N2O, petrol; U_rel = 216.3 % (95 %). Best value 3.3578e-7 kt/TJ with standard uncertainty u = 3.7056 × 10⁻⁷ kt/TJ (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 1.96: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 4282.04 kt
- standard uncertainty u = 835.16 kt

Reference: JCGM GUM-5:2026, 5.4.6, (5.6), (5.8) and Table 5.3: E(CO₂e) = 4282.04 kt, u = 835.16 kt.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="gum5-e5-ghg-inventory-agriculture"></a>

### Greenhouse gas inventory, mobile machinery in agriculture (UK 2018): independent inputs (JCGM GUM-5 Example 5)

`gum5-e5-ghg-inventory-agriculture`

#### What is this project about?

CO₂-equivalent emissions E = Σ s_i A_i F_i (IPCC category A4ci) and their standard uncertainty by the LPU for independent inputs (5.5).


#### Step 1. What do we measure, and how is it calculated?

The result is **E** in kt (Total CO₂-equivalent emissions of the sub-sector). It is not measured directly: it is calculated from the input quantities with the measurement model

```
E = 11/3*A1*F1 + 25*A2*F2 + 298*A3*F3 + 11/3*A4*F4 + 25*A5*F5 + 298*A6*F6
```


#### Step 2. The input quantities and their uncertainties

The model has 12 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **A1**: Activity data, gas oil (CO2); U_rel = 38.6 % (95 %). Best value 56317.092 TJ with standard uncertainty u = 11091 TJ (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **F1**: Emission factor, CO2, gas oil; U_rel = 2.7 % (95 %). Best value 0.020438 kt/TJ with standard uncertainty u = 0.00028154 kt/TJ (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **A2**: Activity data, gas oil (CH4); U_rel = 1.6 % (95 %). Best value 56317.092 TJ with standard uncertainty u = 459.73 TJ (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **F2**: Emission factor, CH4, gas oil; U_rel = 80.0 % (95 %). Best value 3.5368 × 10⁻⁶ kt/TJ with standard uncertainty u = 1.4436 × 10⁻⁶ kt/TJ (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
5. **A3**: Activity data, gas oil (N2O); U_rel = 1.6 % (95 %). Best value 56317.092 TJ with standard uncertainty u = 459.73 TJ (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
6. **F3**: Emission factor, N2O, gas oil; U_rel = 216.3 % (95 %). Best value 3.0984 × 10⁻⁶ kt/TJ with standard uncertainty u = 3.4193 × 10⁻⁶ kt/TJ (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
7. **A4**: Activity data, petrol (CO2); U_rel = 50.7 % (95 %). Best value 67.1856 TJ with standard uncertainty u = 17.379 TJ (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
8. **F4**: Emission factor, CO2, petrol; U_rel = 2.0 % (95 %). Best value 0.019127 kt/TJ with standard uncertainty u = 0.00019517 kt/TJ (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
9. **A5**: Activity data, petrol (CH4); U_rel = 1.6 % (95 %). Best value 67.1856 TJ with standard uncertainty u = 0.54845 TJ (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
10. **F5**: Emission factor, CH4, petrol; U_rel = 80.0 % (95 %). Best value 4.8654 × 10⁻⁵ kt/TJ with standard uncertainty u = 1.9859 × 10⁻⁵ kt/TJ (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
11. **A6**: Activity data, petrol (N2O); U_rel = 1.6 % (95 %). Best value 67.1856 TJ with standard uncertainty u = 0.54845 TJ (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
12. **F6**: Emission factor, N2O, petrol; U_rel = 216.3 % (95 %). Best value 3.3578e-7 kt/TJ with standard uncertainty u = 3.7056 × 10⁻⁷ kt/TJ (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 1.96: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 4282.04 kt
- standard uncertainty u = 835.16 kt

Reference: JCGM GUM-5:2026, 5.4.6, (5.6), (5.8) and Table 5.3: E(CO₂e) = 4282.04 kt, u = 835.16 kt.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Breaking force from destructive tests (Intermediate)

Six wire ropes are pulled until they break. Only six data: a t distribution and a careful look at the Monte Carlo scale.

<a id="gum5-e15-wire-rope-breaking-force"></a>

### Breaking force of steel wire rope from six destructive tests (JCGM GUM-5 Example 15)

`gum5-e15-wire-rope-breaking-force`

#### What is this project about?

F₀ = F̄ − B − B_ref − e_res − e_T: mean of six tests on pieces of one coil, with rectangular corrections for the load cells, resolution and temperature.


#### Step 1. What do we measure, and how is it calculated?

The result is **F_0** in kN (Breaking force of the rope). It is not measured directly: it is calculated from the input quantities with the measurement model

```
F_0 = F - B - B_ref - e_res - e_T
```


#### Step 2. The input quantities and their uncertainties

The model has 5 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **F**: Breaking forces of six pieces. Measured 6 times (a **Type A** evaluation, from statistics). The estimate is the mean 10005.7 kN; the readings scatter with a standard deviation s = 1.75 kN, and the standard uncertainty of the mean is u = s/√n = 1.75/√6 = 0.715 kN, with ν = n − 1 = 5 degrees of freedom.
2. **B**: Error of the test machine load cell. Only limits are known: the value lies between -6.8 and 6.8 kN (0 ± 6.8), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 6.8/√3 = 3.93 kN.
3. **B_ref**: Error of the reference load cell. Only limits are known: the value lies between -8.2 and 8.2 kN (0 ± 8.2), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 8.2/√3 = 4.73 kN.
4. **e_res**: Resolution of the force measuring system. Only limits are known: the value lies between -8.7 and 8.7 kN (0 ± 8.7), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 8.7/√3 = 5.02 kN.
5. **e_T**: Temperature effect. Only limits are known: the value lies between -10.2 and 10.2 kN (0 ± 10.2), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 10.2/√3 = 5.89 kN.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 10005.7 kN
- standard uncertainty u = 9.9 kN
- expanded uncertainty U = 19.4 kN

Reference: JCGM GUM-5:2026, 15.4.1: F₀ = 10 005.7 kN, u = 9.9 kN, U = 19.4 kN (k = 1.96). The MCM standard uncertainty is recomputed: 9.93 kN for the stated inputs; the guide prints 10.1 kN (see note).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Vacuum gauge: what to do with a known correction (Intermediate)

A correction factor can be applied, ignored, or put into the uncertainty. The three scenarios and the effect of a common temperature error on a pressure difference are compared.

<a id="emue-e2.2-vacuum-gauge-correction-applied"></a>

### Vacuum gauge calibration, correction factor and conformance probability, scenario 1 (EMUE E2.2)

`emue-e2.2-vacuum-gauge-correction-applied`

#### What is this project about?

Scenario 1: the reference correction Δp_m = 0.05 % p_std is applied, u(Δp_m) = 0.5 Pa. Point 8 of the calibration (256 Pa); specification: error of reading within ±0.5 %, i.e. 0.995 ≤ f ≤ 1.005.


#### Step 1. What do we measure, and how is it calculated?

The result is **f** (Correction factor of the gauge). It is not measured directly: it is calculated from the input quantities with the measurement model

```
f = (p_std + dp_m)/p_UUC
```


#### Step 2. The input quantities and their uncertainties

The model has 3 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **p_std**: Reference pressure (indication of the reference gauge). Best value 256.4 Pa. The source (e.g. a certificate) gives the expanded uncertainty U = 1.5 Pa with k = 2, so the standard uncertainty is u = U/k = 1.5/2 = 0.750 Pa (normal distribution).
2. **p_UUC**: Indication of the gauge under calibration. Best value 255.9 Pa. The source (e.g. a certificate) gives the expanded uncertainty U = 0.23 Pa with k = 2, so the standard uncertainty is u = U/k = 0.23/2 = 0.115 Pa (normal distribution).
3. **dp_m**: the reference correction Δp_m = 0.05 % p_std is applied, u(Δp_m) = 0.5 Pa. Best value 0.1282 Pa with standard uncertainty u = 0.5 Pa (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Then: does the result meet the specification?

The measured quantity must be between 0.995 and 1.005. Because every measured value is uncertain, a result close to a limit could be on either side of it. The program calculates the **probability of conformity** and applies a **decision rule** (JCGM 106).


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 1.0025
- expanded uncertainty U = 0.0071

Reference: EMUE E2.2, point 8: f = 1.0025, U(f) = 0.0071, p_c = 74.5 % (Tables E2.2.2 and E2.2.3).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="emue-e2.2-vacuum-gauge-correction-ignored"></a>

### Vacuum gauge calibration, correction factor and conformance probability, scenario 3 (EMUE E2.2)

`emue-e2.2-vacuum-gauge-correction-ignored`

#### What is this project about?

Scenario 3: the correction and its uncertainty are ignored. Point 8 of the calibration (256 Pa); specification: error of reading within ±0.5 %, i.e. 0.995 ≤ f ≤ 1.005.


#### Step 1. What do we measure, and how is it calculated?

The result is **f** (Correction factor of the gauge). It is not measured directly: it is calculated from the input quantities with the measurement model

```
f = p_std/p_UUC
```


#### Step 2. The input quantities and their uncertainties

The model has 2 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **p_std**: Reference pressure (indication of the reference gauge). Best value 256.4 Pa. The source (e.g. a certificate) gives the expanded uncertainty U = 1.5 Pa with k = 2, so the standard uncertainty is u = U/k = 1.5/2 = 0.750 Pa (normal distribution).
2. **p_UUC**: Indication of the gauge under calibration. Best value 255.9 Pa. The source (e.g. a certificate) gives the expanded uncertainty U = 0.23 Pa with k = 2, so the standard uncertainty is u = U/k = 0.23/2 = 0.115 Pa (normal distribution).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Then: does the result meet the specification?

The measured quantity must be between 0.995 and 1.005. Because every measured value is uncertain, a result close to a limit could be on either side of it. The program calculates the **probability of conformity** and applies a **decision rule** (JCGM 106).


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 1.002
- expanded uncertainty U = 0.0059

Reference: EMUE E2.2, point 8: f = 1.0020, U(f) = 0.0059, p_c = 83.8 % (Tables E2.2.6 and E2.2.7).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="emue-e2.2-vacuum-gauge-correction-in-uncertainty"></a>

### Vacuum gauge calibration, correction factor and conformance probability, scenario 2 (EMUE E2.2)

`emue-e2.2-vacuum-gauge-correction-in-uncertainty`

#### What is this project about?

Scenario 2: the correction is not applied but folded into its uncertainty, u² = 0.5² + (0.05 % p_std)²/3. Point 8 of the calibration (256 Pa); specification: error of reading within ±0.5 %, i.e. 0.995 ≤ f ≤ 1.005.


#### Step 1. What do we measure, and how is it calculated?

The result is **f** (Correction factor of the gauge). It is not measured directly: it is calculated from the input quantities with the measurement model

```
f = (p_std + dp_m)/p_UUC
```


#### Step 2. The input quantities and their uncertainties

The model has 3 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **p_std**: Reference pressure (indication of the reference gauge). Best value 256.4 Pa. The source (e.g. a certificate) gives the expanded uncertainty U = 1.5 Pa with k = 2, so the standard uncertainty is u = U/k = 1.5/2 = 0.750 Pa (normal distribution).
2. **p_UUC**: Indication of the gauge under calibration. Best value 255.9 Pa. The source (e.g. a certificate) gives the expanded uncertainty U = 0.23 Pa with k = 2, so the standard uncertainty is u = U/k = 0.23/2 = 0.115 Pa (normal distribution).
3. **dp_m**: the correction is not applied but folded into its uncertainty, u² = 0.5² + (0.05 % p_std)²/3. Best value 0 Pa with standard uncertainty u = 0.505449 Pa (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Then: does the result meet the specification?

The measured quantity must be between 0.995 and 1.005. Because every measured value is uncertain, a result close to a limit could be on either side of it. The program calculates the **probability of conformity** and applies a **decision rule** (JCGM 106).


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 1.002
- expanded uncertainty U = 0.0071

Reference: EMUE E2.2, point 8: f = 1.0020, U(f) = 0.0071, p_c = 77.8 % (Tables E2.2.4 and E2.2.5).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="emue-e2.2-vacuum-gauge-sum-model-correlated"></a>

### Vacuum gauge, pressure difference with a common temperature error: correlation included (EMUE E2.2.7)

`emue-e2.2-vacuum-gauge-sum-model-correlated`

#### What is this project about?

Δp = p_UUC − (p_std + Δp_m) with p = p′(1 + αΔT) for both gauges (E2.2.8); point 8, specification ±0.5 % of the reading.


#### Step 1. What do we measure, and how is it calculated?

The result is **dp** in Pa (Pressure difference (error of the gauge)). It is not measured directly: it is calculated from the input quantities with the measurement model

```
dp = p_UUC_obs*(1 + a_UUC*dT) - (p_std_obs*(1 + a_std*dT) + dp_m)
```


#### Step 2. The input quantities and their uncertainties

The model has 6 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **p_std_obs**: Observed reference pressure. Best value 256.4 Pa. The source (e.g. a certificate) gives the expanded uncertainty U = 1.5 Pa with k = 2, so the standard uncertainty is u = U/k = 1.5/2 = 0.750 Pa (normal distribution).
2. **p_UUC_obs**: Observed indication of the gauge under calibration. Best value 255.9 Pa. The source (e.g. a certificate) gives the expanded uncertainty U = 0.23 Pa with k = 2, so the standard uncertainty is u = U/k = 0.23/2 = 0.115 Pa (normal distribution).
3. **a_std**: Temperature coefficient, reference. Best value 0.0033333 1/K with standard uncertainty u = 0.00033333 1/K (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **a_UUC**: Temperature coefficient, gauge under calibration. Best value 0.0033333 1/K with standard uncertainty u = 0.00033333 1/K (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
5. **dT**: Systematic error of the gas temperature. Best value 0 K with standard uncertainty u = 0.57 K (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
6. **dp_m**: Reference pressure correction 0.05 % p_std. Best value 0.1282 Pa with standard uncertainty u = 0.5 Pa (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Then: does the result meet the specification?

The measured quantity must be between -1.2795 and 1.2795. Because every measured value is uncertain, a result close to a limit could be on either side of it. The program calculates the **probability of conformity** and applies a **decision rule** (JCGM 106).


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = -0.6282 Pa
- standard uncertainty u = 0.909 Pa

Reference: EMUE E2.2.7: u(p_std) = 0.90 Pa, u(p_UUC) = 0.51 Pa; u(Δp) = 1.14 Pa without and 0.91 Pa with the correlation; p_c = 0.67 and 0.75.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="emue-e2.2-vacuum-gauge-sum-model-correlation-ignored"></a>

### Vacuum gauge, pressure difference with a common temperature error: correlation ignored (EMUE E2.2.7)

`emue-e2.2-vacuum-gauge-sum-model-correlation-ignored`

#### What is this project about?

Δp = p_UUC − (p_std + Δp_m) with p = p′(1 + αΔT) for both gauges (E2.2.8); point 8, specification ±0.5 % of the reading.


#### Step 1. What do we measure, and how is it calculated?

The result is **dp** in Pa (Pressure difference (error of the gauge)). It is not measured directly: it is calculated from the input quantities with the measurement model

```
dp = p_UUC_obs*(1 + a_UUC*dT_UUC) - (p_std_obs*(1 + a_std*dT_std) + dp_m)
```


#### Step 2. The input quantities and their uncertainties

The model has 7 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **p_std_obs**: Observed reference pressure. Best value 256.4 Pa. The source (e.g. a certificate) gives the expanded uncertainty U = 1.5 Pa with k = 2, so the standard uncertainty is u = U/k = 1.5/2 = 0.750 Pa (normal distribution).
2. **p_UUC_obs**: Observed indication of the gauge under calibration. Best value 255.9 Pa. The source (e.g. a certificate) gives the expanded uncertainty U = 0.23 Pa with k = 2, so the standard uncertainty is u = U/k = 0.23/2 = 0.115 Pa (normal distribution).
3. **a_std**: Temperature coefficient, reference. Best value 0.0033333 1/K with standard uncertainty u = 0.00033333 1/K (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **a_UUC**: Temperature coefficient, gauge under calibration. Best value 0.0033333 1/K with standard uncertainty u = 0.00033333 1/K (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
5. **dT_std**: Systematic error of the gas temperature (treated, wrongly, as separate for each gauge). Best value 0 K with standard uncertainty u = 0.57 K (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
6. **dT_UUC**: Systematic error of the gas temperature (treated, wrongly, as separate for each gauge). Best value 0 K with standard uncertainty u = 0.57 K (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
7. **dp_m**: Reference pressure correction 0.05 % p_std. Best value 0.1282 Pa with standard uncertainty u = 0.5 Pa (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Then: does the result meet the specification?

The measured quantity must be between -1.2795 and 1.2795. Because every measured value is uncertain, a result close to a limit could be on either side of it. The program calculates the **probability of conformity** and applies a **decision rule** (JCGM 106).


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = -0.6282 Pa
- standard uncertainty u = 1.14 Pa

Reference: EMUE E2.2.7: u(p_std) = 0.90 Pa, u(p_UUC) = 0.51 Pa; u(Δp) = 1.14 Pa without and 0.91 Pa with the correlation; p_c = 0.67 and 0.75.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Lead in soil from validation data (Intermediate)

An ISO 21748 budget built from the validation of an ICP-OES method: precision, trueness and further terms.

<a id="emue-e3.4-soil-lead-iso21748-rms-bias"></a>

### Aqua regia extractable lead in soil by ICP-OES: ISO 21748 budget from validation data, root-mean-square bias uncertainty (EMUE E3.4)

`emue-e3.4-soil-lead-iso21748-rms-bias`

#### What is this project about?

u²(w)/w² = u²(δ̂)/w² + s_I²/w² + (c_T u_T)² + (c_t u_t)² + (u(γ₀)/γ₀)² with relative summary values from precision, trueness and ruggedness studies; u(δ̂)/w = 0.035.


#### Step 1. What do we measure, and how is it calculated?

The result is **w** in mg/kg (Aqua regia extractable Pb in soil). It is not measured directly: it is calculated from the input quantities with the measurement model

```
w = w_obs*(1 + d_bias)*(1 + e_I)*(1 + 0.004*dT)*(1 + 0.0009*dt)*(g0/984.74)
```


#### Step 2. The input quantities and their uncertainties

The model has 6 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **w_obs**: Measured mass fraction (result for Pb in C279a, E3.4.7). An exact value, 27.8 mg/kg, with no uncertainty (for example a defined constant or a nominal value).
2. **d_bias**: Relative bias term; its estimate (≈ −10 %) is not applied, its uncertainty is. Best value 0 with standard uncertainty u = 0.035 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **e_I**: Relative intermediate precision (root-mean-square RSD). Best value 0 with standard uncertainty u = 0.036 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **dT**: Deviation of the extraction temperature from 65 °C. Only limits are known: the value lies between -5 and 5 °C (0 ± 5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 5/√3 = 2.89 °C.
5. **dt**: Deviation of the extraction time from 120 min. Only limits are known: the value lies between -5 and 5 min (0 ± 5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 5/√3 = 2.89 min.
6. **g0**: Pb stock solution (certificate). Best value 984.74 mg/kg with standard uncertainty u = 0.985 mg/kg (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 27.8 mg/kg
- standard uncertainty u = 1.4344 mg/kg
- expanded uncertainty U = 2.869 mg/kg

Reference: EMUE E3.4, (E3.4.18) and E3.4.7: u′(Pb)² = 0.002 526, u′ ≈ 5 %, result (27.8 ± 2.8) mg/kg (k = 2); this variant recomputes u(δ̂)/w as the root mean square (F87).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="emue-e3.4-soil-lead-iso21748"></a>

### Aqua regia extractable lead in soil by ICP-OES: ISO 21748 budget from validation data (EMUE E3.4)

`emue-e3.4-soil-lead-iso21748`

#### What is this project about?

u²(w)/w² = u²(δ̂)/w² + s_I²/w² + (c_T u_T)² + (c_t u_t)² + (u(γ₀)/γ₀)² with relative summary values from precision, trueness and ruggedness studies; u(δ̂)/w = 0.033.


#### Step 1. What do we measure, and how is it calculated?

The result is **w** in mg/kg (Aqua regia extractable Pb in soil). It is not measured directly: it is calculated from the input quantities with the measurement model

```
w = w_obs*(1 + d_bias)*(1 + e_I)*(1 + 0.004*dT)*(1 + 0.0009*dt)*(g0/984.74)
```


#### Step 2. The input quantities and their uncertainties

The model has 6 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **w_obs**: Measured mass fraction (result for Pb in C279a, E3.4.7). An exact value, 27.8 mg/kg, with no uncertainty (for example a defined constant or a nominal value).
2. **d_bias**: Relative bias term; its estimate (≈ −10 %) is not applied, its uncertainty is. Best value 0 with standard uncertainty u = 0.033 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **e_I**: Relative intermediate precision (root-mean-square RSD). Best value 0 with standard uncertainty u = 0.036 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **dT**: Deviation of the extraction temperature from 65 °C. Only limits are known: the value lies between -5 and 5 °C (0 ± 5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 5/√3 = 2.89 °C.
5. **dt**: Deviation of the extraction time from 120 min. Only limits are known: the value lies between -5 and 5 min (0 ± 5), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 5/√3 = 2.89 min.
6. **g0**: Pb stock solution (certificate). Best value 984.74 mg/kg with standard uncertainty u = 0.985 mg/kg (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 27.8 mg/kg
- standard uncertainty u = 1.3972 mg/kg
- expanded uncertainty U = 2.794 mg/kg

Reference: EMUE E3.4, (E3.4.18) and E3.4.7: u′(Pb)² = 0.002 526, u′ ≈ 5 %, result (27.8 ± 2.8) mg/kg (k = 2).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## A gas mixture by permeation (Intermediate)

The amount fraction of ammonia from a permeation tube: mass loss rate, flow and molar masses.

<a id="emue-e3.8-ammonia-permeation"></a>

### Calibration gas mixture of NH₃ in N₂ by permeation: amount fraction of ammonia (EMUE E3.8)

`emue-e3.8-ammonia-permeation`

#### What is this project about?

y = ṅ₁x₁/(ṅ₁ + ṅ₂) with ṅ_j = q_mj/M̄_j (E3.8.1): permeation rate from a magnetic suspension balance, carrier flow from a thermal mass flow controller, purities of both parent gases and molar masses from standard atomic weights.


#### Step 1. What do we measure, and how is it calculated?

The result is **y_NH3** in µmol/mol (Amount fraction of NH₃ in the mixture). It is not measured directly: it is calculated from the input quantities with the measurement model

```
y_NH3 = 1e6*n1*(1 - x_w1)/(n1 + n2)
```

To keep the equation readable, 6 intermediate quantities are calculated first:

- `M_NH3 = A_N + 3*A_H`
- `M_H2O = A_H*2 + A_O`
- `M1 = (1 - x_w1)*M_NH3 + x_w1*M_H2O`
- `M2 = (1 - 1e-6*(x2_Ar + x2_CH4 + x2_CO + x2_CO2 + x2_H2 + x2_H2O + x2_O2))*(2*A_N) + 1e-6*(x2_Ar*(A_Ar) + x2_CH4*(A_C + 4*A_H) + x2_CO*(A_C + A_O) + x2_CO2*(A_C + 2*A_O) + x2_H2*(2*A_H) + x2_H2O*(A_H*2 + A_O) + x2_O2*(2*A_O))`
- `n1 = 1e-6*q_m1/M1`
- `n2 = q_m2/M2`


#### Step 2. The input quantities and their uncertainties

The model has 15 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **q_m1**: Permeation rate of the NH₃ tube at 62 °C (OLS slope of mass against time). Best value 13.599 µg/min with standard uncertainty u = 0.108 µg/min (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **q_m2**: Mass flow of the nitrogen carrier gas (thermal MFC, 250 mL/min). Best value 0.3126 g/min with standard uncertainty u = 0.0006 g/min (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **x_w1**: Water in the ammonia (parent 1); x(NH₃) = 1 − x_w1. Best value 0.01 mol/mol with standard uncertainty u = 0.002 mol/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **A_H**: Standard atomic weight of H. Only limits are known: the value lies between 1.00784 and 1.00811 (1.007975 ± 0.00013500), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.00013500/√3 = 7.79 × 10⁻⁵.
5. **A_C**: Standard atomic weight of C. Only limits are known: the value lies between 12.0096 and 12.0116 (12.0106 ± 0.0010000), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.0010000/√3 = 0.000577.
6. **A_N**: Standard atomic weight of N. Only limits are known: the value lies between 14.00643 and 14.00728 (14.006855 ± 0.00042500), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.00042500/√3 = 0.000245.
7. **A_O**: Standard atomic weight of O. Only limits are known: the value lies between 15.99903 and 15.99977 (15.9994 ± 0.00037000), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.00037000/√3 = 0.000214.
8. **A_Ar**: Standard atomic weight of Ar (IUPAC 2013, 39.948 ± 0.001, as in Table E3.8.4). Only limits are known: the value lies between 39.947 and 39.949 (39.948 ± 0.001), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.001/√3 = 0.000577.
9. **x2_Ar**: Ar in the nitrogen (parent 2). Best value 5 µmol/mol with standard uncertainty u = 3 µmol/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
10. **x2_CH4**: CH4 in the nitrogen (parent 2). Best value 0.008 µmol/mol with standard uncertainty u = 0.005 µmol/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
11. **x2_CO**: CO in the nitrogen (parent 2). Best value 0.015 µmol/mol with standard uncertainty u = 0.009 µmol/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
12. **x2_CO2**: CO2 in the nitrogen (parent 2). Best value 0.01 µmol/mol with standard uncertainty u = 0.006 µmol/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
13. **x2_H2**: H2 in the nitrogen (parent 2). Best value 0.025000 µmol/mol with standard uncertainty u = 0.015 µmol/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
14. **x2_H2O**: H2O in the nitrogen (parent 2). Best value 0.01 µmol/mol with standard uncertainty u = 0.006 µmol/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
15. **x2_O2**: O2 in the nitrogen (parent 2). Best value 0.005 µmol/mol with standard uncertainty u = 0.003 µmol/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 70.796 µmol/mol
- standard uncertainty u = 0.598 µmol/mol
- expanded uncertainty U = 1.2 µmol/mol

Reference: EMUE E3.8.6 and Table E3.8.5: y(NH₃) = 7.08·10⁻⁵ mol/mol, u = 5.99·10⁻⁷ mol/mol; (70.8 ± 1.2) µmol/mol (k = 2).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Anti-doping: ephedrine in urine (Intermediate)

A top-down uncertainty from intermediate precision and bias, and the decision limit used by WADA to declare an adverse finding.

<a id="emue-e6.5-ephedrine-wada-decision-limit"></a>

### Ephedrine in urine: WADA decision limit as a guarded-rejection rule (EMUE E6.5, JCGM 106)

`emue-e6.5-ephedrine-wada-decision-limit`

#### What is this project about?

Threshold T = 10 µg/mL; an adverse analytical finding requires the concentration to exceed T with at least 95 % confidence, using the target uncertainty u_c,max = 0.5 µg/mL: DL = T + 1.645 u_c,max.


#### Step 1. The idea: does the item meet the specification?

The measured quantity γ(ephedrine) must be at most 10 µg/mL. Because every measured value is uncertain, a result close to a limit could be on either side of it. The program calculates the **probability of conformity** and applies a **decision rule** (JCGM 106).

Decision rule: guarded rejection: an item is rejected only if it is clearly outside the limit (e.g. in law enforcement or anti-doping, where a false accusation must be avoided).


#### Finally: what the source expects

Reference: EMUE E6.5: DL = T + 1.645 u_c,max, set at 11 µg/mL for T = 10 µg/mL and u_c,max = 0.5 µg/mL.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="emue-e6.5-ephedrine-wada-topdown"></a>

### Ephedrine in urine for anti-doping testing: top-down uncertainty from intermediate precision and a CRM (EMUE E6.5)

`emue-e6.5-ephedrine-wada-topdown`

#### What is this project about?

u_c(x) = √(s_w²/n + u_bias²) for the mean of triplicate results, with u_bias² = s_ref²/n′ + Δ² + u²(x_ref) from 30 analyses of a CRM (Table E6.5.1); WADA requires u_c ≤ 0.5 µg/mL (5 %) near the threshold of 10 µg/mL.


#### Step 1. The idea: uncertainty from the laboratory's own data

This is a **top-down** evaluation of **γ(ephedrine)** (µg/mL). Instead of modelling every step of the method, it uses data that the laboratory already has: how much the results scatter when the method is repeated on different days (precision) and how far they are, on average, from the true value (bias or recovery). Together these cover most sources of uncertainty at once.

The uncertainty is evaluated at 1 concentration levels: 10 µg/mL.


#### Step 2. The data

**Precision** (random effects), 1 entry:

- **u(Rw), mean of 3**: a standard deviation of 2.424871 % (e.g. from a control chart: within-laboratory reproducibility).

**Bias** (systematic effects): 1 source, e.g. certified reference materials or proficiency tests. From the differences between the laboratory's results and the reference values, the program estimates u(bias), which includes the uncertainty of the reference values themselves.


#### Step 3. What the program calculates

It combines the components as u = √(u(precision)² + u(bias)² + …), at each level, and multiplies by the coverage factor (k = 2 for about 95 %) to get the expanded uncertainty U. The results show u and U, absolute and relative, at every level.


> **Try:** Compare the size of the precision and bias components in the results. If bias dominates, a better reference material or a correction helps more than repeating measurements.


#### Finally: what the source expects

Reference: EMUE E6.5, Table E6.5.1: u_prec = 2.4 %, u_bias = 3.0 %, u_c = 3.8 % (recomputed 3.89 % without rounding, F91).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Uncertainty from reproducibility data (Intermediate)

When a standard method has been tested in an interlaboratory study, its reproducibility standard deviation is a good first estimate of the uncertainty (ISO 21748). The examples add further terms where needed.

<a id="iso21748-c2-meat-content"></a>

### Meat content from nitrogen and fat (ISO 21748 C.2)

`iso21748-c2-meat-content`

#### What is this project about?

ISO 21748:2017 C.2: a combined result w_meat = 100 w_N/f_N + w_fat whose inputs each carry reproducibility-based uncertainties (Clause 12), written as a measurement model with relative error terms.


#### Step 1. What do we measure, and how is it calculated?

The result is **w_meat** in % (Total meat content). It is not measured directly: it is calculated from the input quantities with the measurement model

```
w_meat = 100 * w_N * (1 + e_L + e_r) / f_N + w_fat * (1 + e_fat)
```


#### Step 2. The input quantities and their uncertainties

The model has 6 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **w_N**: Total nitrogen content (Kjeldahl). An exact value, 3.29 %, with no uncertainty (for example a defined constant or a nominal value).
2. **e_L**: Relative laboratory effect, s_L = 0.011. Best value 0 with standard uncertainty u = 0.011 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **e_r**: Relative repeatability of the mean of duplicates, 0.018/√2. Best value 0 with standard uncertainty u = 0.0127279 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **f_N**: Nitrogen factor of the meat. Best value 3.65 with standard uncertainty u = 0.052 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
5. **w_fat**: Fat content. An exact value, 5.5 %, with no uncertainty (for example a defined constant or a nominal value).
6. **e_fat**: Relative reproducibility of the fat result, s_R = 0.02. Best value 0 with standard uncertainty u = 0.02 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 95.6 %
- standard uncertainty u = 1.98 %
- expanded uncertainty U = 4 %

Reference: ISO 21748:2017 Table C.1 and C.2.6 (w_meat = 95.6 %, u = 1.98 %, U = 4.0 %).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="iso21748-c4-crude-fibre"></a>

### Crude fibre in animal feed from reproducibility data (ISO 21748 C.4)

`iso21748-c4-crude-fibre`

#### What is this project about?

ISO 21748:2017 C.4: level-dependent reproducibility from a collaborative study plus the uncertainty of drying to constant mass; the same example as QUAM A6 with a different treatment of the drying term.


#### Step 1. The idea: uncertainty from the laboratory's own data

This is a **top-down** evaluation of **w(crude fibre)** (% m/m). Instead of modelling every step of the method, it uses data that the laboratory already has: how much the results scatter when the method is repeated on different days (precision) and how far they are, on average, from the true value (bias or recovery). Together these cover most sources of uncertainty at once.

The uncertainty is evaluated at 3 concentration levels: 2.3; 4; 8 % m/m.


#### Step 2. The data

**Precision** (random effects), 3 entries:

- **s_R (≤ 2.5)**: repeatability and reproducibility from an interlaboratory study (ISO 5725 / ISO 21748).
- **s_R (2.5–5)**: repeatability and reproducibility from an interlaboratory study (ISO 5725 / ISO 21748).
- **s_R (5–10)**: repeatability and reproducibility from an interlaboratory study (ISO 5725 / ISO 21748).

**Further components**, added as they are given:

- drying to constant mass


#### Step 3. What the program calculates

It combines the components as u = √(u(precision)² + u(bias)² + …), at each level, and multiplies by the coverage factor (k = 2 for about 95 %) to get the expanded uncertainty U. The results show u and U, absolute and relative, at every level.


> **Try:** Compare the size of the precision and bias components in the results. If bias dominates, a better reference material or a correction helps more than repeating measurements.


#### Finally: what the source expects

Reference: ISO 21748:2017 Table C.9 and C.4.9 (u = 0.31, 0.41, 0.59 %; U = 0.6, 0.8, 1.2 %).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="iso21748-c1-co-emissions"></a>

### Carbon monoxide in vehicle emissions from reproducibility data (ISO 21748 C.1)

`iso21748-c1-co-emissions`

#### What is this project about?

ISO 21748:2017 C.1: type-approval test of CO emissions; with bias and precision under control the reproducibility standard deviation of the collaborative study is the standard uncertainty.


#### Step 1. The idea: uncertainty from the laboratory's own data

This is a **top-down** evaluation of **CO** (g/km). Instead of modelling every step of the method, it uses data that the laboratory already has: how much the results scatter when the method is repeated on different days (precision) and how far they are, on average, from the true value (bias or recovery). Together these cover most sources of uncertainty at once.

The uncertainty is evaluated at 1 concentration levels: 2.2 g/km.


#### Step 2. The data

**Precision** (random effects), 1 entry:

- **s_R**: repeatability and reproducibility from an interlaboratory study (ISO 5725 / ISO 21748).


#### Step 3. What the program calculates

It combines the components as u = √(u(precision)² + u(bias)² + …), at each level, and multiplies by the coverage factor (k = 2 for about 95 %) to get the expanded uncertainty U. The results show u and U, absolute and relative, at every level.


> **Try:** Compare the size of the precision and bias components in the results. If bias dominates, a better reference material or a correction helps more than repeating measurements.


#### Finally: what the source expects

Reference: ISO 21748:2017 C.1.2 and C.1.6 (s_r = 0.22, s_R = 0.28 g/km, U = 0.56 g/km).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="iso21748-c3-aerobic-plate-count-flour"></a>

### Aerobic plate count, flour: adjusted reproducibility (ISO 21748 C.3)

`iso21748-c3-aerobic-plate-count-flour`

#### What is this project about?

ISO 21748:2017 C.3 (AOAC 990.12, flour): relative uncertainty of log10(count) from collaborative-study s_R and s_r, the laboratory's better repeatability (7.3.2) and a sample-preparation term.


#### Step 1. The idea: uncertainty from the laboratory's own data

This is a **top-down** evaluation of **log10(CFU), flour**. Instead of modelling every step of the method, it uses data that the laboratory already has: how much the results scatter when the method is repeated on different days (precision) and how far they are, on average, from the true value (bias or recovery). Together these cover most sources of uncertainty at once.


#### Step 2. The data

**Precision** (random effects), 1 entry:

- **s′_R**: repeatability and reproducibility from an interlaboratory study (ISO 5725 / ISO 21748).

**Further components**, added as they are given:

- sample preparation


#### Step 3. What the program calculates

It combines the components as u = √(u(precision)² + u(bias)² + …), at each level, and multiplies by the coverage factor (k = 2 for about 95 %) to get the expanded uncertainty U. The results show u and U, absolute and relative, at every level.


> **Try:** Compare the size of the precision and bias components in the results. If bias dominates, a better reference material or a correction helps more than repeating measurements.


#### Finally: what the source expects

Reference: ISO 21748:2017 Tables C.3 and C.4 and C.3.7 (flour: s′_R = 5.5 %, u = 6.4 %, U = 12.8 %); flour: recomputed, see note.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="iso21748-c3-aerobic-plate-count-shrimp"></a>

### Aerobic plate count, shrimp: adjusted reproducibility (ISO 21748 C.3)

`iso21748-c3-aerobic-plate-count-shrimp`

#### What is this project about?

ISO 21748:2017 C.3 (AOAC 990.12, shrimp): relative uncertainty of log10(count) from collaborative-study s_R and s_r, the laboratory's better repeatability (7.3.2) and a sample-preparation term.


#### Step 1. The idea: uncertainty from the laboratory's own data

This is a **top-down** evaluation of **log10(CFU), shrimp**. Instead of modelling every step of the method, it uses data that the laboratory already has: how much the results scatter when the method is repeated on different days (precision) and how far they are, on average, from the true value (bias or recovery). Together these cover most sources of uncertainty at once.


#### Step 2. The data

**Precision** (random effects), 1 entry:

- **s′_R**: repeatability and reproducibility from an interlaboratory study (ISO 5725 / ISO 21748).

**Further components**, added as they are given:

- sample preparation


#### Step 3. What the program calculates

It combines the components as u = √(u(precision)² + u(bias)² + …), at each level, and multiplies by the coverage factor (k = 2 for about 95 %) to get the expanded uncertainty U. The results show u and U, absolute and relative, at every level.


> **Try:** Compare the size of the precision and bias components in the results. If bias dominates, a better reference material or a correction helps more than repeating measurements.


#### Finally: what the source expects

Reference: ISO 21748:2017 Tables C.3 and C.4 and C.3.7 (shrimp: s′_R = 7.2 %, u = 7.8 %, U = 15.6 %).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="iso21748-c3-aerobic-plate-count-vegetables"></a>

### Aerobic plate count, vegetables: adjusted reproducibility (ISO 21748 C.3)

`iso21748-c3-aerobic-plate-count-vegetables`

#### What is this project about?

ISO 21748:2017 C.3 (AOAC 990.12, vegetables): relative uncertainty of log10(count) from collaborative-study s_R and s_r, the laboratory's better repeatability (7.3.2) and a sample-preparation term.


#### Step 1. The idea: uncertainty from the laboratory's own data

This is a **top-down** evaluation of **log10(CFU), vegetables**. Instead of modelling every step of the method, it uses data that the laboratory already has: how much the results scatter when the method is repeated on different days (precision) and how far they are, on average, from the true value (bias or recovery). Together these cover most sources of uncertainty at once.


#### Step 2. The data

**Precision** (random effects), 1 entry:

- **s′_R**: repeatability and reproducibility from an interlaboratory study (ISO 5725 / ISO 21748).

**Further components**, added as they are given:

- sample preparation


#### Step 3. What the program calculates

It combines the components as u = √(u(precision)² + u(bias)² + …), at each level, and multiplies by the coverage factor (k = 2 for about 95 %) to get the expanded uncertainty U. The results show u and U, absolute and relative, at every level.


> **Try:** Compare the size of the precision and bias components in the results. If bias dominates, a better reference material or a correction helps more than repeating measurements.


#### Finally: what the source expects

Reference: ISO 21748:2017 Tables C.3 and C.4 and C.3.7 (vegetables: s′_R = 8.4 %, u = 8.9 %, U = 17.8 %).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## An interlaboratory precision study (Intermediate)

Eight or more laboratories, several levels, replicate results: repeatability and reproducibility of a method, after Cochran's and Grubbs's tests. Each example is evaluated classically and with REML.

<a id="iso5725-2-c1-sulfur-in-coal-reml"></a>

### Sulfur content of coal: REML estimates (ISO 5725-2 Example C.1)

`iso5725-2-c1-sulfur-in-coal-reml`

#### What is this project about?

ISO 5725-2:2019 C.1.9: the data of Example C.1 evaluated by restricted maximum likelihood (Annex B.2) instead of Formulae (23)–(31).


#### Step 1. The idea: an interlaboratory precision study

8 laboratories measured **Sulfur content of coal** in replicate at 4 levels (ISO 5725-2). From this, the program calculates the **repeatability** s_r (same laboratory, same day) and the **reproducibility** s_R (different laboratories) of the method.

Before that, outlier tests (Cochran for variances, Grubbs for means) and Mandel's h and k statistics show laboratories whose results do not fit. The method is REML (restricted maximum likelihood), which handles unbalanced data.


#### Finally: what the source expects

Reference: ISO 5725-2:2019, Table C.6.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="iso5725-2-c1-sulfur-in-coal"></a>

### Sulfur content of coal: repeatability and reproducibility (ISO 5725-2 Example C.1)

`iso5725-2-c1-sulfur-in-coal`

#### What is this project about?

ISO 5725-2:2019 C.1: a uniform-level experiment with eight laboratories at four levels (0.69 % to 3.25 %), no missing or outlying data, unequal numbers of results per cell.


#### Step 1. The idea: an interlaboratory precision study

8 laboratories measured **Sulfur content of coal** in replicate at 4 levels (ISO 5725-2). From this, the program calculates the **repeatability** s_r (same laboratory, same day) and the **reproducibility** s_R (different laboratories) of the method.

Before that, outlier tests (Cochran for variances, Grubbs for means) and Mandel's h and k statistics show laboratories whose results do not fit. The method is the classical analysis of variance of ISO 5725-2.


#### Finally: what the source expects

Reference: ISO 5725-2:2019, Formulae (C.1)–(C.4), Tables C.4 and C.5.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="iso5725-2-c3-creosote-oil-reml"></a>

### Thermometric titration of creosote oil: REML estimates (ISO 5725-2 Example C.3)

`iso5725-2-c3-creosote-oil-reml`

#### What is this project about?

ISO 5725-2:2019 C.3.8: the retained data of Example C.3 evaluated by restricted maximum likelihood (Annex B.2).


#### Step 1. The idea: an interlaboratory precision study

9 laboratories measured **Creosote oil, thermometric titration** in replicate at 5 levels (ISO 5725-2). From this, the program calculates the **repeatability** s_r (same laboratory, same day) and the **reproducibility** s_R (different laboratories) of the method.

Before that, outlier tests (Cochran for variances, Grubbs for means) and Mandel's h and k statistics show laboratories whose results do not fit. The method is REML (restricted maximum likelihood), which handles unbalanced data.


#### Finally: what the source expects

Reference: ISO 5725-2:2019, Table C.19.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="iso5725-2-c3-creosote-oil"></a>

### Thermometric titration of creosote oil: outlying laboratory and precision relationships (ISO 5725-2 Example C.3)

`iso5725-2-c3-creosote-oil`

#### What is this project about?

ISO 5725-2:2019 C.3 with 8.5: nine laboratories at five levels (4 % to 20 %), an outlying laboratory, a rejected cell, and the functional relationships I–IV between the precision and the level (Tables 1–4).


#### Step 1. The idea: an interlaboratory precision study

9 laboratories measured **Creosote oil, thermometric titration** in replicate at 5 levels (ISO 5725-2). From this, the program calculates the **repeatability** s_r (same laboratory, same day) and the **reproducibility** s_R (different laboratories) of the method.

Before that, outlier tests (Cochran for variances, Grubbs for means) and Mandel's h and k statistics show laboratories whose results do not fit. The method is the classical analysis of variance of ISO 5725-2.


#### Finally: what the source expects

Reference: ISO 5725-2:2019, Tables C.17 and C.18, Tables 1–4, and C.3.5.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="iso5725-2-c2-softening-point-pitch-reml"></a>

### Softening point of pitch: REML estimates (ISO 5725-2 Example C.2)

`iso5725-2-c2-softening-point-pitch-reml`

#### What is this project about?

ISO 5725-2:2019 C.2.9: the data of Example C.2 evaluated by restricted maximum likelihood (Annex B.2).


#### Step 1. The idea: an interlaboratory precision study

16 laboratories measured **Softening point of pitch** in replicate at 4 levels (ISO 5725-2). From this, the program calculates the **repeatability** s_r (same laboratory, same day) and the **reproducibility** s_R (different laboratories) of the method.

Before that, outlier tests (Cochran for variances, Grubbs for means) and Mandel's h and k statistics show laboratories whose results do not fit. The method is REML (restricted maximum likelihood), which handles unbalanced data.


#### Finally: what the source expects

Reference: ISO 5725-2:2019, Table C.13.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="iso5725-2-c2-softening-point-pitch"></a>

### Softening point of pitch: repeatability and reproducibility with missing data (ISO 5725-2 Example C.2)

`iso5725-2-c2-softening-point-pitch`

#### What is this project about?

ISO 5725-2:2019 C.2: ring-and-ball softening point of pitch, sixteen laboratories at four levels (88 °C to 102 °C) with duplicate results and two missing cells.


#### Step 1. The idea: an interlaboratory precision study

16 laboratories measured **Softening point of pitch** in replicate at 4 levels (ISO 5725-2). From this, the program calculates the **repeatability** s_r (same laboratory, same day) and the **reproducibility** s_R (different laboratories) of the method.

Before that, outlier tests (Cochran for variances, Grubbs for means) and Mandel's h and k statistics show laboratories whose results do not fit. The method is the classical analysis of variance of ISO 5725-2.


#### Finally: what the source expects

Reference: ISO 5725-2:2019, Formulae (C.25)–(C.33), Tables C.10–C.12.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## A straight-line calibration (Intermediate)

Nitrogen monoxide by UV absorption calibrated with a straight line through reference gas mixtures, with uncertainties on both axes.

<a id="iso6143-b1-no-uv-linear"></a>

### NO in N₂ by UV absorption: straight-line analysis function (ISO 6143 Example 1)

`iso6143-b1-no-uv-linear`

#### What is this project about?

ISO 6143:2025 B.2.2: three reference gas mixtures (5 µmol/mol to 50 µmol/mol) with uncertainties in both the contents and the UV responses, a straight-line analysis function by generalized least squares and the contents of three mixtures with their covariances.


#### Step 1. The idea: a calibration curve

An instrument gives a response (here in absorbance) that is not directly the content of **NO**. It is calibrated with 3 reference mixtures of known content. Both the contents and the responses have uncertainties, so the fit (generalised least squares, ISO 6143) uses both.

Analysis function: **linear**. The program checks whether it fits the points (the weighted residuals and Γ should be small).


#### Step 2. Unknown samples

For 3 samples, the measured response is converted into a content through the fitted function; the uncertainty includes both the response of the sample and the uncertainty of the curve.


#### Finally: what the source expects

Reference: ISO 6143:2025 B.2.2, parameters and Tables B.4 and B.6 (B_LEAST).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Purity by qNMR (TÜBİTAK UME) (Intermediate)

An illustrative budget for quantitative NMR with an internal standard, from the TÜBİTAK UME uncertainty procedure.

<a id="ume-procedure-19-2-qnmr-purity-internal-standard"></a>

### Purity by ¹H qNMR with an internal standard: an illustrative budget (TÜBİTAK UME uncertainty procedure, Example 19.2)

`ume-procedure-19-2-qnmr-purity-internal-standard`

#### What is this project about?

Multiplicative model with relative uncertainties; the values are illustrative, not from a real measurement. The integration is a separate Type B component (±0.2 %, rectangular) beside the repeatability of the ratio.


#### Step 1. What do we measure, and how is it calculated?

The result is **P_a** in mg/g (Purity (mass fraction) of the analyte by ¹H qNMR). It is not measured directly: it is calculated from the input quantities with the measurement model

```
P_a = R * f_int * (N_std/N_a) * (M_a/M_std) * (m_std/m_a) * P_std
```


#### Step 2. The input quantities and their uncertainties

The model has 9 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **R**: Integral ratio I_a/I_std, 5 independent samples, RSD 0.15 % (Type A). Best value 1.40023 1 with standard uncertainty u = 0.000939 1. Because it is based on few data (ν = 4 degrees of freedom), a Student t distribution is used: it has heavier tails than the normal distribution and makes the coverage factor larger.
2. **f_int**: Integration (phase, baseline, limits), ±0.2 %. Only limits are known: the value lies between 0.998 and 1.002 1 (1 ± 0.002), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.002/√3 = 0.00115 1.
3. **N_std**: ¹H nuclei of the internal-standard signal (2, e.g. maleic acid). An exact value, 2 1, with no uncertainty (for example a defined constant or a nominal value).
4. **N_a**: ¹H nuclei of the analyte signal (3, a methyl singlet). An exact value, 3 1, with no uncertainty (for example a defined constant or a nominal value).
5. **M_a**: Molar mass of the analyte (IUPAC atomic weights). Only limits are known: the value lies between 194.18 and 194.20 g/mol (194.19 ± 0.0070633), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.0070633/√3 = 0.00408 g/mol.
6. **M_std**: Molar mass of the internal standard. Only limits are known: the value lies between 116.07 and 116.08 g/mol (116.072 ± 0.0034177), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.0034177/√3 = 0.00197 g/mol.
7. **m_std**: Mass of the internal standard, microbalance u = 2.0 µg. Best value 6.512 mg with standard uncertainty u = 0.002 mg (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
8. **m_a**: Mass of the analyte, microbalance u = 2.0 µg. Best value 10.215 mg with standard uncertainty u = 0.002 mg (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
9. **P_std**: Purity of the internal-standard CRM, U = 1.5 mg/g (k = 2). Best value 999.4 mg/g. The source (e.g. a certificate) gives the expanded uncertainty U = 1.5 mg/g with k = 2, so the standard uncertainty is u = U/k = 1.5/2 = 0.750 mg/g (normal distribution).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 995 mg/g
- standard uncertainty u = 1.567 mg/g
- expanded uncertainty U = 3.13 mg/g

Reference: UME procedure, Section 19.2: P_a = 995.0 mg/g, relative u_c = 0.001 575, u_c = 1.567 mg/g, ν_eff = 121, U = 3.13 mg/g → (995.0 ± 3.1) mg/g (k = 2).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Sampling: nitrate in lettuce (Intermediate)

The duplicate method: two samples from each of eight lots, each analysed twice. The sampling part of the uncertainty is larger than the analytical part.

<a id="ufs-a1-nitrate-lettuce"></a>

### Nitrate in glasshouse lettuce: duplicate method, classical ANOVA (Eurachem UfS Example A1)

`ufs-a1-nitrate-lettuce`

#### What is this project about?

Eight bays of lettuce, each sampled twice (10-head composite samples, W pattern) with duplicate analyses by HPLC (Table A1.1).


#### Step 1. The idea: the uncertainty starts at sampling

A result describes a whole lot, field or batch, but only a small sample is analysed. The **duplicate method** (Eurachem UfS) takes two independent samples from 8 targets and analyses each twice. An analysis of variance then separates the variation due to **sampling** from that due to **analysis**.


> **Try:** Compare u(sampling) with u(analysis) in the results: often the sampling part is the larger one, and a better analysis would not help.


#### Finally: what the source expects

Reference: Classical ANOVA results printed in Figure A1.2 (the robust results are not compared).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Sampling: lead in soil (Intermediate)

A large relative uncertainty is better expressed as a factor (multiply or divide) on a log scale.

<a id="ufs-a2-lead-soil-uncertainty-factor"></a>

### Lead in contaminated top soil: classical ANOVA and uncertainty factor (Eurachem UfS Example A2)

`ufs-a2-lead-soil-uncertainty-factor`

#### What is this project about?

Ten of 100 grid targets (30 m × 30 m, 0–150 mm) sampled in duplicate 3 m apart and analysed twice by ICP-AES (Table A2.2). The values are skewed: the uncertainty is also expressed as a factor from the ANOVA of ln x.


#### Step 1. The idea: the uncertainty starts at sampling

A result describes a whole lot, field or batch, but only a small sample is analysed. The **duplicate method** (Eurachem UfS) takes two independent samples from 10 targets and analyses each twice. An analysis of variance then separates the variation due to **sampling** from that due to **analysis**.


> **Try:** Compare u(sampling) with u(analysis) in the results: often the sampling part is the larger one, and a better analysis would not help.


#### Finally: what the source expects

Reference: Classical ANOVA of Figure A2.2 and of the logarithms in Table A2.3; F_U and u′ of Sections 6.2 and 6.3.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Sampling: iron in groundwater (Intermediate)

The same data evaluated with the analysis of variance and with the simpler range method.

<a id="ufs-a3-iron-groundwater-anova"></a>

### Dissolved iron in groundwater: classical ANOVA (Eurachem UfS Example A3)

`ufs-a3-iron-groundwater-anova`

#### What is this project about?

Six monitoring wells of a groundwater body, each sampled twice (on-line filtered and preserved) with duplicate laboratory analyses by ICP-AES (Table A3.7).


#### Step 1. The idea: the uncertainty starts at sampling

A result describes a whole lot, field or batch, but only a small sample is analysed. The **duplicate method** (Eurachem UfS) takes two independent samples from 6 targets and analyses each twice. An analysis of variance then separates the variation due to **sampling** from that due to **analysis**.


> **Try:** Compare u(sampling) with u(analysis) in the results: often the sampling part is the larger one, and a better analysis would not help.


#### Finally: what the source expects

Reference: ANOVA row of Table A3.5 (the RANOVA row, 1.8 %, 9.9 % and 72 %, is not compared).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="ufs-a3-iron-groundwater-range"></a>

### Dissolved iron in groundwater: range method (Eurachem UfS Example A3, corrected)

`ufs-a3-iron-groundwater-range`

#### What is this project about?

Six monitoring wells of a groundwater body, each sampled twice (on-line filtered and preserved) with duplicate laboratory analyses by ICP-AES (Table A3.7).


#### Step 1. The idea: the uncertainty starts at sampling

A result describes a whole lot, field or batch, but only a small sample is analysed. The **duplicate method** (Eurachem UfS) takes two independent samples from 6 targets and analyses each twice. An analysis of variance then separates the variation due to **sampling** from that due to **analysis**.


> **Try:** Compare u(sampling) with u(analysis) in the results: often the sampling part is the larger one, and a better analysis would not help.


#### Finally: what the source expects

Reference: Range calculations of Table A3.7 as corrected by errata 1.1 (s_T+S+A = 0.604, s_T = 0.601 mg/L, CV_T = 35 %); Table A3.4 and summary of the errata.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Sampling: vitamin A in porridge (Intermediate)

Small test portions make the sampling variance large; with 4 g portions the estimated analytical variance becomes negative, which shows the limits of the method. A control chart of duplicates is included.

<a id="ufs-a4-vitamin-a-porridge-40g"></a>

### Vitamin A in baby porridge, 40 g test samples: duplicate method (Eurachem UfS Example A4)

`ufs-a4-vitamin-a-porridge-40g`

#### What is this project about?

Two samples (packages) from each of ten production batches of an oat porridge with bananas and apricots, each split into duplicate test samples for HPLC (no analytical duplicates of one test sample).


#### Step 1. The idea: the uncertainty starts at sampling

A result describes a whole lot, field or batch, but only a small sample is analysed. The **duplicate method** (Eurachem UfS) takes two independent samples from 10 targets and analyses each twice. An analysis of variance then separates the variation due to **sampling** from that due to **analysis**.


> **Try:** Compare u(sampling) with u(analysis) in the results: often the sampling part is the larger one, and a better analysis would not help.


#### Finally: what the source expects

Reference: Tables A4.5, A4.6 and A4.14–A4.15; U′ of the summary table and RSD_B = 6.1 % as corrected by errata 1.4a.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="ufs-a4-vitamin-a-porridge-4g"></a>

### Vitamin A in baby porridge, 4 g test samples: negative sampling variance (Eurachem UfS Example A4)

`ufs-a4-vitamin-a-porridge-4g`

#### What is this project about?

Two samples (packages) from each of ten production batches of an oat porridge with bananas and apricots, each split into duplicate test samples for HPLC (no analytical duplicates of one test sample).


#### Step 1. The idea: the uncertainty starts at sampling

A result describes a whole lot, field or batch, but only a small sample is analysed. The **duplicate method** (Eurachem UfS) takes two independent samples from 10 targets and analyses each twice. An analysis of variance then separates the variation due to **sampling** from that due to **analysis**.


> **Try:** Compare u(sampling) with u(analysis) in the results: often the sampling part is the larger one, and a better analysis would not help.


#### Finally: what the source expects

Reference: Tables A4.8 and A4.9 (V_samp = −2662.15 set to zero) and the analytical U′ of Table A4.10.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="ufs-a4-vitamin-a-qc-chart"></a>

### Vitamin A in baby porridge: control chart of duplicate samples (Eurachem UfS Example A4)

`ufs-a4-vitamin-a-qc-chart`

#### What is this project about?

Quality control of sampling: two samples from one batch of each of eight porridges, each analysed twice (40 g); the relative difference D % of the two samples is charted against limits from the validated uncertainty (Table A4.13, Figure A4.4).


#### Step 1. The idea: watching a method over time

A **control chart** (Nordtest TR 569) plots the result of a control sample, measured with every series, against time. Warning limits (±2 s) and action limits (±3 s) around a central line show when the method is no longer in statistical control.

Limits: target limits, from the precision the method must reach.

16 control values are entered.


> **Try:** In the results, look for points beyond the action limits and for two of three successive points beyond the warning limits: these are the rules that say "stop and find the cause".


#### Finally: what the source expects

Reference: Limits of Section 7.3 (CL 11 %, WL 27 %, AL 36 %) and Figure A4.4: no value outside the warning limit.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Sampling uncertainty by modelling (Intermediate)

Instead of duplicates, the sampling steps are modelled one by one, as in a GUM budget.

<a id="ufs-a6-cadmium-soil"></a>

### Cadmium in agricultural top soil: modelling approach (Eurachem UfS Example A6)

`ufs-a6-cadmium-soil`

#### What is this project about?

Mean concentration of an arable field of 0.32 ha (143 m × 22 m, 0–30 cm) from composite samples; uncertainty budget of the sampling effects (between locations, strategy, depth, splitting, drying) and of the analysis.


#### Step 1. What do we measure, and how is it calculated?

The result is **x_site** in mg/kg (Mean cadmium content of the field). It is not measured directly: it is calculated from the input quantities with the measurement model

```
x_site = x_sqr * f_strat * f_depth * f_split * f_dry * f_Rw * f_ref * f_bias
```


#### Step 2. The input quantities and their uncertainties

The model has 8 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **x_sqr**: Cadmium in the composite samples of five squares A1, A3, B2, C1, C3 (Table A6.1). Measured 5 times (a **Type A** evaluation, from statistics). The estimate is the mean 0.319200 mg/kg; the readings scatter with a standard deviation s = 0.0387 mg/kg, and the standard uncertainty of the mean is u = s/√n = 0.0387/√5 = 0.0173 mg/kg, with ν = n − 1 = 4 degrees of freedom.
2. **f_strat**: Sampling strategy (bias from the spatial pattern). Best value 1 with standard uncertainty u = 0.01 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **f_depth**: 'Depth effect': ±10 % in depth with the gradient of Table A6.2, rectangular. Only limits are known: the value lies between 0.9376 and 1.0624 (1 ± 0.0624), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.0624/√3 = 0.0360.
4. **f_split**: Splitting by coning and quartering: mean RSD of duplicate splits (Table A6.3). Best value 1 with standard uncertainty u = 0.037 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
5. **f_dry**: Moisture of air-dried soil, a range of 2 %, rectangular. Only limits are known: the value lies between 0.99 and 1.01 (1 ± 0.01), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.01/√3 = 0.00577.
6. **f_Rw**: Within-laboratory reproducibility (mean of 10 test samples, GF-ZAAS). Best value 1 with standard uncertainty u = 0.036 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
7. **f_ref**: Certified value of the CRM. Best value 1 with standard uncertainty u = 0.027 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
8. **f_bias**: Day-to-day bias correction with the CRM (mean of 3). Best value 1 with standard uncertainty u = 0.027 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 200 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 200 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 0.32 mg/kg
- expanded uncertainty U = 0.06 mg/kg

Reference: Measurement result and expanded uncertainty of Section 6.7 (k = 2).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="ufs-a6-phosphorus-soil"></a>

### Phosphorus in agricultural top soil: modelling approach (Eurachem UfS Example A6)

`ufs-a6-phosphorus-soil`

#### What is this project about?

Mean concentration of an arable field of 0.32 ha (143 m × 22 m, 0–30 cm) from composite samples; uncertainty budget of the sampling effects (between locations, strategy, depth, splitting, drying) and of the analysis.


#### Step 1. What do we measure, and how is it calculated?

The result is **x_site** in mg/kg (Mean phosphorus content of the field). It is not measured directly: it is calculated from the input quantities with the measurement model

```
x_site = x_sqr * f_strat * f_depth * f_split * f_dry * f_Rw * f_bias
```


#### Step 2. The input quantities and their uncertainties

The model has 7 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **x_sqr**: Phosphorus in the composite samples of five squares A1, A3, B2, C1, C3 (Table A6.1). Measured 5 times (a **Type A** evaluation, from statistics). The estimate is the mean 115.800 mg/kg; the readings scatter with a standard deviation s = 7.43 mg/kg, and the standard uncertainty of the mean is u = s/√n = 7.43/√5 = 3.32 mg/kg, with ν = n − 1 = 4 degrees of freedom.
2. **f_strat**: Sampling strategy (bias from the spatial pattern). Best value 1 with standard uncertainty u = 0.005 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **f_depth**: 'Depth effect': ±10 % in depth with the gradient of Table A6.2, rectangular. Only limits are known: the value lies between 0.93528 and 1.0647 (1 ± 0.06472), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.06472/√3 = 0.0374.
4. **f_split**: Splitting by coning and quartering: mean RSD of duplicate splits (Table A6.3). Best value 1 with standard uncertainty u = 0.0316 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
5. **f_dry**: Moisture of air-dried soil, a range of 2 %, rectangular. Only limits are known: the value lies between 0.99 and 1.01 (1 ± 0.01), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.01/√3 = 0.00577.
6. **f_Rw**: Within-laboratory reproducibility (CAL method). Best value 1 with standard uncertainty u = 0.017 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
7. **f_bias**: Trueness from the reproducibility s_R of an interlaboratory comparison. Best value 1 with standard uncertainty u = 0.095 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 200 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 200 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 116 mg/kg
- expanded uncertainty U = 26 mg/kg

Reference: Measurement result and expanded uncertainty of Section 6.7 (k = 2).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Global risks of false decisions (Advanced)

With knowledge of the production process, the probability that an accepted item is actually bad (consumer's risk) can be calculated for the whole production, and an acceptance limit chosen for a required risk.

<a id="jcgm106-9.5-ball-bearings"></a>

### Ball bearings: acceptance limit for a required consumer's risk (JCGM 106 9.5.4)

`jcgm106-9.5-ball-bearings`

#### What is this project about?

JCGM 106:2012 9.5.4 Example: radial error motion ≤ 2 µm, gamma process PDF with mean 1 µm and standard deviation 0.5 µm, measuring system u_m = 0.25 µm; global consumer's risk at most 0.1 %.


#### Step 1. The idea: does the item meet the specification?

The measured quantity radial error motion must be at most 2 µm. Because every measured value is uncertain, a result close to a limit could be on either side of it. The program calculates the **probability of conformity** and applies a **decision rule** (JCGM 106).

Decision rule: guarded acceptance: the acceptance zone is made smaller than the tolerance by a guard band, so that accepting a bad item is unlikely (protects the customer).

Information about the production process (the spread of items before measurement) is also used, so that the **global risks** can be calculated: the fraction of all accepted items that are actually bad (consumer's risk) and of rejected items that are actually good (producer's risk).


#### Finally: what the source expects

Reference: JCGM 106:2012 9.5.4, Example (4.2 % non-conforming, r ≈ 0.65, A ≈ 1.7 µm, R_P ≈ 7.5 %).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="jcgm106-9.5-precision-resistors"></a>

### Precision resistors: global consumer's and producer's risks (JCGM 106 9.5.3)

`jcgm106-9.5-precision-resistors`

#### What is this project about?

JCGM 106:2012 9.5.3 Example: 1500 Ω ± 0.2 Ω resistors from a process with σ = 0.12 Ω, inspected with u_m = 0.04 Ω and acceptance limits 1499.82 Ω and 1500.18 Ω.


#### Step 1. The idea: does the item meet the specification?

The measured quantity R must be between 1499.8 and 1500.2 Ω. Because every measured value is uncertain, a result close to a limit could be on either side of it. The program calculates the **probability of conformity** and applies a **decision rule** (JCGM 106).

Information about the production process (the spread of items before measurement) is also used, so that the **global risks** can be calculated: the fraction of all accepted items that are actually bad (consumer's risk) and of rejected items that are actually good (producer's risk).


#### Finally: what the source expects

Reference: JCGM 106:2012 9.5.3, Example (p_c ≈ 90 %, R_C ≈ 1 %, R_P ≈ 7 %).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Several components must conform (Advanced)

A medicine with four correlated active components: the risks of a false decision are calculated jointly by Monte Carlo.

<a id="emue-e2.1-multicomponent-medication"></a>

### Total risks of false decisions for a medication with four correlated active components (EMUE E2.1)

`emue-e2.1-multicomponent-medication`

#### What is this project about?

NyQuil tablets: APAP, DEX, DOX and PE, tolerance 95–105 % of the labelled amount, u_rel = 2.8 %; total specific consumer's risks for three sets of test results and the total global consumer's risk.


#### Step 1. The idea: several properties must conform at once

An item conforms only if all 4 components are within their limits. Because the measured values are correlated, the risks of a wrong decision are calculated jointly (Monte Carlo).


#### Finally: what the source expects

Reference: EMUE E2.1.6.2–E2.1.6.3: R*_tot = 0.0029, 0.0117, 0.0002; R_tot = 0.0018 (10⁷ trials; 0.0019 in the original paper).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Global and specific risks with a prior (Advanced)

Particulate matter in air: prior knowledge of typical concentrations (a lognormal distribution) is combined with the measurement to calculate the risks of a decision.

<a id="gum5-e10-tspm-global-risks-acceptance-0.17"></a>

### Suspended particulate matter in air: global risks with a lognormal prior, A_U = 0.17 mg/m³ (JCGM GUM-5 Example 10)

`gum5-e10-tspm-global-risks-acceptance-0.17`

#### What is this project about?

T_U = 0.2 mg/m³, lognormal population PDF, relative uncertainty 7 %; acceptance limit moved to 0.17 mg/m³.


#### Step 1. The idea: does the item meet the specification?

The measured quantity ρ must be at most 0.2 mg/m³. Because every measured value is uncertain, a result close to a limit could be on either side of it. The program calculates the **probability of conformity** and applies a **decision rule** (JCGM 106).

Information about the production process (the spread of items before measurement) is also used, so that the **global risks** can be calculated: the fraction of all accepted items that are actually bad (consumer's risk) and of rejected items that are actually good (producer's risk).


#### Finally: what the source expects

Reference: JCGM GUM-5:2026, 10.6.3.3–10.6.3.4 print R_C = 0.002 %, R_P = 5.8 %; the expected values are for the conditional measuring-system PDF (F80).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="gum5-e10-tspm-global-risks-acceptance-0.22"></a>

### Suspended particulate matter in air: global risks with a lognormal prior, A_U = 0.22 mg/m³ (JCGM GUM-5 Example 10)

`gum5-e10-tspm-global-risks-acceptance-0.22`

#### What is this project about?

T_U = 0.2 mg/m³, lognormal population PDF, relative uncertainty 7 %; acceptance limit moved to 0.22 mg/m³.


#### Step 1. The idea: does the item meet the specification?

The measured quantity ρ must be at most 0.2 mg/m³. Because every measured value is uncertain, a result close to a limit could be on either side of it. The program calculates the **probability of conformity** and applies a **decision rule** (JCGM 106).

Information about the production process (the spread of items before measurement) is also used, so that the **global risks** can be calculated: the fraction of all accepted items that are actually bad (consumer's risk) and of rejected items that are actually good (producer's risk).


#### Finally: what the source expects

Reference: JCGM GUM-5:2026, 10.6.3.3–10.6.3.4 print R_C = 1.7 %, R_P = < 0.1 %; the expected values are for the conditional measuring-system PDF (F80).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="gum5-e10-tspm-global-risks-shared-risk"></a>

### Suspended particulate matter in air: global risks with a lognormal prior, A_U = 0.2 mg/m³ (JCGM GUM-5 Example 10)

`gum5-e10-tspm-global-risks-shared-risk`

#### What is this project about?

T_U = 0.2 mg/m³, lognormal population PDF, relative uncertainty 7 %; shared-risk rule A_U = T_U.


#### Step 1. The idea: does the item meet the specification?

The measured quantity ρ must be at most 0.2 mg/m³. Because every measured value is uncertain, a result close to a limit could be on either side of it. The program calculates the **probability of conformity** and applies a **decision rule** (JCGM 106).

Information about the production process (the spread of items before measurement) is also used, so that the **global risks** can be calculated: the fraction of all accepted items that are actually bad (consumer's risk) and of rejected items that are actually good (producer's risk).


#### Finally: what the source expects

Reference: JCGM GUM-5:2026, 10.6.3.3–10.6.3.4 print R_C = 0.5 %, R_P = 0.9 %; the expected values are for the conditional measuring-system PDF (F80).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="gum5-e10-tspm-specific-risks"></a>

### Suspended particulate matter in air: specific risks from the posterior with a lognormal prior (JCGM GUM-5 Example 10)

`gum5-e10-tspm-specific-risks`

#### What is this project about?

Conformance probabilities of single results from the posterior g(ρ | ρ̂) ∝ g₀(ρ)·h(ρ̂ | ρ), lognormal prior, u = 0.07·ρ̂, A_U = T_U = 0.2 mg/m³.


#### Step 1. The idea: does the item meet the specification?

The measured quantity ρ must be at most 0.2 mg/m³. Because every measured value is uncertain, a result close to a limit could be on either side of it. The program calculates the **probability of conformity** and applies a **decision rule** (JCGM 106).

Information about the production process (the spread of items before measurement) is also used, so that the **global risks** can be calculated: the fraction of all accepted items that are actually bad (consumer's risk) and of rejected items that are actually good (producer's risk).


#### Finally: what the source expects

Reference: JCGM GUM-5:2026, 10.7.2: specific producer's risk about 11 % for ρ̂ = 0.225 mg/m³ (Figure 10.3).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## A strongly nonlinear model (Advanced)

The comparison loss in microwave power: near zero the model is so nonlinear that the first-order GUM fails completely, while Monte Carlo gives the right answer. The variants add correlation and higher-order terms.

<a id="jcgm101-9.4-comparison-loss-x000-correlated-higher-order"></a>

### Comparison loss, x₁ = 0, r = 0.9, higher-order GUM terms for correlated inputs

`jcgm101-9.4-comparison-loss-x000-correlated-higher-order`

#### What is this project about?

JCGM 101 9.4: comparison loss in the calibration of a microwave power meter (values in units of 10⁻⁶).


#### Step 1. What do we measure, and how is it calculated?

The result is **dY** in 10⁻⁶ (Comparison loss). It is not measured directly: it is calculated from the input quantities with the measurement model

```
dY = 1e6 * (X1^2 + X2^2)
```


#### Step 2. The input quantities and their uncertainties

The model has 2 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **X1**: Real part of the reflection coefficient. Best value 0 with standard uncertainty u = 0.005 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **X2**: Imaginary part of the reflection coefficient. Best value 0 with standard uncertainty u = 0.005 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).

**Correlated inputs.** Some inputs are not independent, because a common effect (the same standard, the same instrument, the same temperature) acts on both. The correlation coefficient r tells how strongly; the program adds the corresponding term to the combined uncertainty:

- X1 ↔ X2: r = 0.9


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.
6. Because the model is clearly nonlinear, second-order terms are added (GUM 5.1.2 and its 2026 amendment).

It also uses the **Monte Carlo method**: it draws 2 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 2 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 50 10⁻⁶
- standard uncertainty u = 67.2681 10⁻⁶

Reference: Analytic expectation and standard deviation of δY for bivariate normal inputs (GUM-5 Table 16.2, column A: 50, 67·10⁻⁶); MCM as in the first-order project.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="jcgm101-9.4-comparison-loss-x000-correlated"></a>

### Comparison loss, x₁ = 0.0, r = 0.9

`jcgm101-9.4-comparison-loss-x000-correlated`

#### What is this project about?

JCGM 101 9.4: comparison loss in the calibration of a microwave power meter (values in units of 10⁻⁶).


#### Step 1. What do we measure, and how is it calculated?

The result is **dY** in 10⁻⁶ (Comparison loss). It is not measured directly: it is calculated from the input quantities with the measurement model

```
dY = 1e6 * (X1^2 + X2^2)
```


#### Step 2. The input quantities and their uncertainties

The model has 2 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **X1**: Real part of the reflection coefficient. Best value 0 with standard uncertainty u = 0.005 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **X2**: Imaginary part of the reflection coefficient. Best value 0 with standard uncertainty u = 0.005 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).

**Correlated inputs.** Some inputs are not independent, because a common effect (the same standard, the same instrument, the same temperature) acts on both. The correlation coefficient r tells how strongly; the program adds the corresponding term to the combined uncertainty:

- X1 ↔ X2: r = 0.9


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.

It also uses the **Monte Carlo method**: it draws 2 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 2 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- standard uncertainty u = 0 10⁻⁶

Reference: JCGM 101:2008 Table 9 and an independent 2 × 10⁸-trial reference; tolerances ≈ 5 × engine dispersion at M = 2 × 10⁶.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="jcgm101-9.4-comparison-loss-x000"></a>

### Comparison loss, x₁ = 0.0

`jcgm101-9.4-comparison-loss-x000`

#### What is this project about?

JCGM 101 9.4: comparison loss in the calibration of a microwave power meter (values in units of 10⁻⁶).


#### Step 1. What do we measure, and how is it calculated?

The result is **dY** in 10⁻⁶ (Comparison loss). It is not measured directly: it is calculated from the input quantities with the measurement model

```
dY = 1e6 * (X1^2 + X2^2)
```


#### Step 2. The input quantities and their uncertainties

The model has 2 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **X1**: Real part of the reflection coefficient. Best value 0 with standard uncertainty u = 0.005 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **X2**: Imaginary part of the reflection coefficient. Best value 0 with standard uncertainty u = 0.005 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.
6. Because the model is clearly nonlinear, second-order terms are added (GUM 5.1.2 and its 2026 amendment).

It also uses the **Monte Carlo method**: it draws 2 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 2 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 50 10⁻⁶
- standard uncertainty u = 50 10⁻⁶

Reference: JCGM 101:2008 Table 8 and the analytic non-central χ² solution (Annex F); tolerances ≈ 5 × engine dispersion at M = 2 × 10⁶. The GUM estimate is the analytic one, which the second-order term of JCGM 100:2008/Amd.1:2026 reproduces.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="jcgm101-9.4-comparison-loss-x010-correlated-higher-order"></a>

### Comparison loss, x₁ = 0.01, r = 0.9, higher-order GUM terms for correlated inputs

`jcgm101-9.4-comparison-loss-x010-correlated-higher-order`

#### What is this project about?

JCGM 101 9.4: comparison loss in the calibration of a microwave power meter (values in units of 10⁻⁶).


#### Step 1. What do we measure, and how is it calculated?

The result is **dY** in 10⁻⁶ (Comparison loss). It is not measured directly: it is calculated from the input quantities with the measurement model

```
dY = 1e6 * (X1^2 + X2^2)
```


#### Step 2. The input quantities and their uncertainties

The model has 2 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **X1**: Real part of the reflection coefficient. Best value 0.01 with standard uncertainty u = 0.005 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **X2**: Imaginary part of the reflection coefficient. Best value 0 with standard uncertainty u = 0.005 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).

**Correlated inputs.** Some inputs are not independent, because a common effect (the same standard, the same instrument, the same temperature) acts on both. The correlation coefficient r tells how strongly; the program adds the corresponding term to the combined uncertainty:

- X1 ↔ X2: r = 0.9


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.
6. Because the model is clearly nonlinear, second-order terms are added (GUM 5.1.2 and its 2026 amendment).

It also uses the **Monte Carlo method**: it draws 2 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 2 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 150 10⁻⁶
- standard uncertainty u = 120.5197 10⁻⁶

Reference: Analytic expectation and standard deviation of δY for bivariate normal inputs (GUM-5 Table 16.2, column A: 150, 121·10⁻⁶); MCM as in the first-order project.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="jcgm101-9.4-comparison-loss-x010-correlated"></a>

### Comparison loss, x₁ = 0.01, r = 0.9

`jcgm101-9.4-comparison-loss-x010-correlated`

#### What is this project about?

JCGM 101 9.4: comparison loss in the calibration of a microwave power meter (values in units of 10⁻⁶).


#### Step 1. What do we measure, and how is it calculated?

The result is **dY** in 10⁻⁶ (Comparison loss). It is not measured directly: it is calculated from the input quantities with the measurement model

```
dY = 1e6 * (X1^2 + X2^2)
```


#### Step 2. The input quantities and their uncertainties

The model has 2 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **X1**: Real part of the reflection coefficient. Best value 0.01 with standard uncertainty u = 0.005 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **X2**: Imaginary part of the reflection coefficient. Best value 0 with standard uncertainty u = 0.005 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).

**Correlated inputs.** Some inputs are not independent, because a common effect (the same standard, the same instrument, the same temperature) acts on both. The correlation coefficient r tells how strongly; the program adds the corresponding term to the combined uncertainty:

- X1 ↔ X2: r = 0.9


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.

It also uses the **Monte Carlo method**: it draws 2 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 2 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- standard uncertainty u = 100 10⁻⁶

Reference: JCGM 101:2008 Table 9 and an independent 2 × 10⁸-trial reference; tolerances ≈ 5 × engine dispersion at M = 2 × 10⁶.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="jcgm101-9.4-comparison-loss-x010"></a>

### Comparison loss, x₁ = 0.01

`jcgm101-9.4-comparison-loss-x010`

#### What is this project about?

JCGM 101 9.4: comparison loss in the calibration of a microwave power meter (values in units of 10⁻⁶).


#### Step 1. What do we measure, and how is it calculated?

The result is **dY** in 10⁻⁶ (Comparison loss). It is not measured directly: it is calculated from the input quantities with the measurement model

```
dY = 1e6 * (X1^2 + X2^2)
```


#### Step 2. The input quantities and their uncertainties

The model has 2 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **X1**: Real part of the reflection coefficient. Best value 0.01 with standard uncertainty u = 0.005 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **X2**: Imaginary part of the reflection coefficient. Best value 0 with standard uncertainty u = 0.005 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.
6. Because the model is clearly nonlinear, second-order terms are added (GUM 5.1.2 and its 2026 amendment).

It also uses the **Monte Carlo method**: it draws 2 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 2 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 150 10⁻⁶
- standard uncertainty u = 112 10⁻⁶

Reference: JCGM 101:2008 Table 8 and the analytic non-central χ² solution (Annex F); tolerances ≈ 5 × engine dispersion at M = 2 × 10⁶. The GUM estimate is the analytic one, which the second-order term of JCGM 100:2008/Amd.1:2026 reproduces.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="jcgm101-9.4-comparison-loss-x050-correlated-higher-order"></a>

### Comparison loss, x₁ = 0.05, r = 0.9, higher-order GUM terms for correlated inputs

`jcgm101-9.4-comparison-loss-x050-correlated-higher-order`

#### What is this project about?

JCGM 101 9.4: comparison loss in the calibration of a microwave power meter (values in units of 10⁻⁶).


#### Step 1. What do we measure, and how is it calculated?

The result is **dY** in 10⁻⁶ (Comparison loss). It is not measured directly: it is calculated from the input quantities with the measurement model

```
dY = 1e6 * (X1^2 + X2^2)
```


#### Step 2. The input quantities and their uncertainties

The model has 2 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **X1**: Real part of the reflection coefficient. Best value 0.05 with standard uncertainty u = 0.005 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **X2**: Imaginary part of the reflection coefficient. Best value 0 with standard uncertainty u = 0.005 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).

**Correlated inputs.** Some inputs are not independent, because a common effect (the same standard, the same instrument, the same temperature) acts on both. The correlation coefficient r tells how strongly; the program adds the corresponding term to the combined uncertainty:

- X1 ↔ X2: r = 0.9


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.
6. Because the model is clearly nonlinear, second-order terms are added (GUM 5.1.2 and its 2026 amendment).

It also uses the **Monte Carlo method**: it draws 2 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 2 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 2550.0 10⁻⁶
- standard uncertainty u = 504.5047 10⁻⁶

Reference: Analytic expectation and standard deviation of δY for bivariate normal inputs (GUM-5 Table 16.2, column A: 2550, 505·10⁻⁶); MCM as in the first-order project.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="jcgm101-9.4-comparison-loss-x050-correlated"></a>

### Comparison loss, x₁ = 0.05, r = 0.9

`jcgm101-9.4-comparison-loss-x050-correlated`

#### What is this project about?

JCGM 101 9.4: comparison loss in the calibration of a microwave power meter (values in units of 10⁻⁶).


#### Step 1. What do we measure, and how is it calculated?

The result is **dY** in 10⁻⁶ (Comparison loss). It is not measured directly: it is calculated from the input quantities with the measurement model

```
dY = 1e6 * (X1^2 + X2^2)
```


#### Step 2. The input quantities and their uncertainties

The model has 2 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **X1**: Real part of the reflection coefficient. Best value 0.05 with standard uncertainty u = 0.005 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **X2**: Imaginary part of the reflection coefficient. Best value 0 with standard uncertainty u = 0.005 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).

**Correlated inputs.** Some inputs are not independent, because a common effect (the same standard, the same instrument, the same temperature) acts on both. The correlation coefficient r tells how strongly; the program adds the corresponding term to the combined uncertainty:

- X1 ↔ X2: r = 0.9


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.

It also uses the **Monte Carlo method**: it draws 2 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 2 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- standard uncertainty u = 500 10⁻⁶

Reference: JCGM 101:2008 Table 9 and an independent 2 × 10⁸-trial reference; tolerances ≈ 5 × engine dispersion at M = 2 × 10⁶.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="jcgm101-9.4-comparison-loss-x050"></a>

### Comparison loss, x₁ = 0.05

`jcgm101-9.4-comparison-loss-x050`

#### What is this project about?

JCGM 101 9.4: comparison loss in the calibration of a microwave power meter (values in units of 10⁻⁶).


#### Step 1. What do we measure, and how is it calculated?

The result is **dY** in 10⁻⁶ (Comparison loss). It is not measured directly: it is calculated from the input quantities with the measurement model

```
dY = 1e6 * (X1^2 + X2^2)
```


#### Step 2. The input quantities and their uncertainties

The model has 2 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **X1**: Real part of the reflection coefficient. Best value 0.05 with standard uncertainty u = 0.005 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **X2**: Imaginary part of the reflection coefficient. Best value 0 with standard uncertainty u = 0.005 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.
6. Because the model is clearly nonlinear, second-order terms are added (GUM 5.1.2 and its 2026 amendment).

It also uses the **Monte Carlo method**: it draws 2 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 2 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 2550 10⁻⁶
- standard uncertainty u = 502 10⁻⁶

Reference: JCGM 101:2008 Table 8 and the analytic non-central χ² solution (Annex F); tolerances ≈ 5 × engine dispersion at M = 2 × 10⁶. The GUM estimate is the analytic one, which the second-order term of JCGM 100:2008/Amd.1:2026 reproduces.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Gauge block: exact and approximate models (Advanced)

The full nonlinear gauge-block model of JCGM 101 compared with its linear approximation, by GUM and Monte Carlo.

<a id="jcgm101-9.5-gauge-block-approximate"></a>

### Gauge block calibration, approximate model

`jcgm101-9.5-gauge-block-approximate`

#### What is this project about?

JCGM 101 9.5: the GUM H.1 gauge block with the input PDFs assigned by JCGM 101 (t, arc sine, curvilinear trapezoid); 99 % coverage.


#### Step 1. What do we measure, and how is it calculated?

The result is **dL** in nm (Deviation of the gauge length from 50 mm). It is not measured directly: it is calculated from the input quantities with the measurement model

```
dL = Ls + D + d1 + d2 - Ls*(d_alpha*(theta0 + Delta) + alpha_s*d_theta) - 50000000
```


#### Step 2. The input quantities and their uncertainties

The model has 9 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **Ls**: Length of the standard at 20 °C. Best value 50000623 nm with standard uncertainty u = 25.0 nm. Because it is based on few data (ν = 18 degrees of freedom), a Student t distribution is used: it has heavier tails than the normal distribution and makes the coverage factor larger.
2. **D**: Mean difference of five indications. Best value 215 nm with standard uncertainty u = 6.00 nm. Because it is based on few data (ν = 24 degrees of freedom), a Student t distribution is used: it has heavier tails than the normal distribution and makes the coverage factor larger.
3. **d1**: Random effects of the comparator. Best value 0 nm with standard uncertainty u = 4.00 nm. Because it is based on few data (ν = 5 degrees of freedom), a Student t distribution is used: it has heavier tails than the normal distribution and makes the coverage factor larger.
4. **d2**: Systematic effects of the comparator. Best value 0 nm with standard uncertainty u = 7.00 nm. Because it is based on few data (ν = 8 degrees of freedom), a Student t distribution is used: it has heavier tails than the normal distribution and makes the coverage factor larger.
5. **alpha_s**: Thermal expansion coefficient of the standard. Only limits are known: the value lies between 0.0000095 and 0.0000135 1/°C (0.0000115 ± 2.0000 × 10⁻⁶), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 2.0000 × 10⁻⁶/√3 = 1.15 × 10⁻⁶ 1/°C.
6. **theta0**: Mean temperature deviation. Best value -0.1 °C with standard uncertainty u = 0.2 °C (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
7. **Delta**: Cyclic temperature variation. The value swings between -0.5 and 0.5 °C and is most often near the limits (U-shaped or arcsine distribution; e.g. a temperature cycling around a set point, or a mismatch phase). u = a/√2 = 0.5/√2 = 0.354 °C.
8. **d_alpha**: Difference of expansion coefficients (limits known to ±10 %). The value lies between -0.000001 and 0.000001 1/°C, but the limits themselves are only known to ± 1e-7 (curvilinear trapezoid, JCGM 101 6.4.3). u = √(a²/3 + d²/9) = 5.78 × 10⁻⁷ 1/°C. This standard uncertainty is itself only known roughly: it has 50 degrees of freedom.
9. **d_theta**: Temperature difference of the gauges (limits known to ±50 %). The value lies between -0.05 and 0.05 °C, but the limits themselves are only known to ± 0.025 (curvilinear trapezoid, JCGM 101 6.4.3). u = √(a²/3 + d²/9) = 0.0300 °C. This standard uncertainty is itself only known roughly: it has 2 degrees of freedom.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It chooses the coverage factor k for a coverage probability of 99.0 %, using the effective degrees of freedom (Welch–Satterthwaite), and gives the **expanded uncertainty** U = k·u_c.

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 838 nm
- standard uncertainty u = 32 nm
- coverage factor k = 2.92

Reference: JCGM 101:2008 Table 11 (values rounded to 1 nm).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="jcgm101-9.5-gauge-block-nonlinear"></a>

### Gauge block calibration, non-linear model

`jcgm101-9.5-gauge-block-nonlinear`

#### What is this project about?

JCGM 101 9.5: the GUM H.1 gauge block with the input PDFs assigned by JCGM 101 (t, arc sine, curvilinear trapezoid); 99 % coverage.


#### Step 1. What do we measure, and how is it calculated?

The result is **dL** in nm (Deviation of the gauge length from 50 mm). It is not measured directly: it is calculated from the input quantities with the measurement model

```
dL = (Ls*(1 + alpha_s*(theta0 + Delta - d_theta)) + D + d1 + d2) / (1 + (alpha_s + d_alpha)*(theta0 + Delta)) - 50000000
```


#### Step 2. The input quantities and their uncertainties

The model has 9 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **Ls**: Length of the standard at 20 °C. Best value 50000623 nm with standard uncertainty u = 25.0 nm. Because it is based on few data (ν = 18 degrees of freedom), a Student t distribution is used: it has heavier tails than the normal distribution and makes the coverage factor larger.
2. **D**: Mean difference of five indications. Best value 215 nm with standard uncertainty u = 6.00 nm. Because it is based on few data (ν = 24 degrees of freedom), a Student t distribution is used: it has heavier tails than the normal distribution and makes the coverage factor larger.
3. **d1**: Random effects of the comparator. Best value 0 nm with standard uncertainty u = 4.00 nm. Because it is based on few data (ν = 5 degrees of freedom), a Student t distribution is used: it has heavier tails than the normal distribution and makes the coverage factor larger.
4. **d2**: Systematic effects of the comparator. Best value 0 nm with standard uncertainty u = 7.00 nm. Because it is based on few data (ν = 8 degrees of freedom), a Student t distribution is used: it has heavier tails than the normal distribution and makes the coverage factor larger.
5. **alpha_s**: Thermal expansion coefficient of the standard. Only limits are known: the value lies between 0.0000095 and 0.0000135 1/°C (0.0000115 ± 2.0000 × 10⁻⁶), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 2.0000 × 10⁻⁶/√3 = 1.15 × 10⁻⁶ 1/°C.
6. **theta0**: Mean temperature deviation. Best value -0.1 °C with standard uncertainty u = 0.2 °C (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
7. **Delta**: Cyclic temperature variation. The value swings between -0.5 and 0.5 °C and is most often near the limits (U-shaped or arcsine distribution; e.g. a temperature cycling around a set point, or a mismatch phase). u = a/√2 = 0.5/√2 = 0.354 °C.
8. **d_alpha**: Difference of expansion coefficients (limits known to ±10 %). The value lies between -0.000001 and 0.000001 1/°C, but the limits themselves are only known to ± 1e-7 (curvilinear trapezoid, JCGM 101 6.4.3). u = √(a²/3 + d²/9) = 5.78 × 10⁻⁷ 1/°C. This standard uncertainty is itself only known roughly: it has 50 degrees of freedom.
9. **d_theta**: Temperature difference of the gauges (limits known to ±50 %). The value lies between -0.05 and 0.05 °C, but the limits themselves are only known to ± 0.025 (curvilinear trapezoid, JCGM 101 6.4.3). u = √(a²/3 + d²/9) = 0.0300 °C. This standard uncertainty is itself only known roughly: it has 2 degrees of freedom.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It chooses the coverage factor k for a coverage probability of 99.0 %, using the effective degrees of freedom (Welch–Satterthwaite), and gives the **expanded uncertainty** U = k·u_c.

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 838 nm
- standard uncertainty u = 32 nm
- coverage factor k = 2.92

Reference: JCGM 101:2008 Table 11 (values rounded to 1 nm).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Cartesian to polar coordinates (Advanced)

A magnitude and a phase from two coordinates. Near the origin the transformation is strongly nonlinear and the coverage region is not an ellipse.

<a id="jcgm102-9.3-polar-x001-correlated"></a>

### Cartesian to polar co-ordinates, x₁ = 0.001, r = 0.9 (JCGM 102 9.3)

`jcgm102-9.3-polar-x001-correlated`

#### What is this project about?

JCGM 102:2011 9.3: magnitude and phase of a complex quantity estimated at x₁ = 0.001, x₂ = 0; GUM and Monte Carlo results compared (Table 7).


#### Step 1. What do we measure, and how is it calculated?

This project has 2 results (output quantities) that are calculated together from the same inputs. Each has its own equation:

- **R**: `R = sqrt(X1^2 + X2^2)`
- **Theta** rad: `Theta = atan2(X2, X1)`


#### Step 2. The input quantities and their uncertainties

The model has 2 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **X1**. Best value 0.001 with standard uncertainty u = 0.01 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **X2**. Best value 0 with standard uncertainty u = 0.01 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).

**Correlated inputs.** Some inputs are not independent, because a common effect (the same standard, the same instrument, the same temperature) acts on both. The correlation coefficient r tells how strongly; the program adds the corresponding term to the combined uncertainty:

- X1 ↔ X2: r = 0.9


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It chooses the coverage factor k for a coverage probability of 95.0 %, using the effective degrees of freedom (Welch–Satterthwaite), and gives the **expanded uncertainty** U = k·u_c.

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

Reference: JCGM 102:2011 Table 7, x₁ = 0.001 (MCM with M = 10⁷; tolerances include the Monte Carlo dispersion for 10⁶ trials).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="jcgm102-9.3-polar-x001"></a>

### Cartesian to polar co-ordinates, x₁ = 0.001 (JCGM 102 9.3)

`jcgm102-9.3-polar-x001`

#### What is this project about?

JCGM 102:2011 9.3: magnitude and phase of a complex quantity estimated at x₁ = 0.001, x₂ = 0; GUM and Monte Carlo results compared (Table 6).


#### Step 1. What do we measure, and how is it calculated?

This project has 2 results (output quantities) that are calculated together from the same inputs. Each has its own equation:

- **R**: `R = sqrt(X1^2 + X2^2)`
- **Theta** rad: `Theta = atan2(X2, X1)`


#### Step 2. The input quantities and their uncertainties

The model has 2 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **X1**. Best value 0.001 with standard uncertainty u = 0.01 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **X2**. Best value 0 with standard uncertainty u = 0.01 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It chooses the coverage factor k for a coverage probability of 95.0 %, using the effective degrees of freedom (Welch–Satterthwaite), and gives the **expanded uncertainty** U = k·u_c.

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

Reference: JCGM 102:2011 Table 6, x₁ = 0.001 (MCM with M = 10⁷; tolerances include the Monte Carlo dispersion for 10⁶ trials).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="jcgm102-9.3-polar-x010-correlated"></a>

### Cartesian to polar co-ordinates, x₁ = 0.01, r = 0.9 (JCGM 102 9.3)

`jcgm102-9.3-polar-x010-correlated`

#### What is this project about?

JCGM 102:2011 9.3: magnitude and phase of a complex quantity estimated at x₁ = 0.01, x₂ = 0; GUM and Monte Carlo results compared (Table 7).


#### Step 1. What do we measure, and how is it calculated?

This project has 2 results (output quantities) that are calculated together from the same inputs. Each has its own equation:

- **R**: `R = sqrt(X1^2 + X2^2)`
- **Theta** rad: `Theta = atan2(X2, X1)`


#### Step 2. The input quantities and their uncertainties

The model has 2 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **X1**. Best value 0.01 with standard uncertainty u = 0.01 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **X2**. Best value 0 with standard uncertainty u = 0.01 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).

**Correlated inputs.** Some inputs are not independent, because a common effect (the same standard, the same instrument, the same temperature) acts on both. The correlation coefficient r tells how strongly; the program adds the corresponding term to the combined uncertainty:

- X1 ↔ X2: r = 0.9


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It chooses the coverage factor k for a coverage probability of 95.0 %, using the effective degrees of freedom (Welch–Satterthwaite), and gives the **expanded uncertainty** U = k·u_c.

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

Reference: JCGM 102:2011 Table 7, x₁ = 0.01 (MCM with M = 10⁷; tolerances include the Monte Carlo dispersion for 10⁶ trials).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="jcgm102-9.3-polar-x010"></a>

### Cartesian to polar co-ordinates, x₁ = 0.01 (JCGM 102 9.3)

`jcgm102-9.3-polar-x010`

#### What is this project about?

JCGM 102:2011 9.3: magnitude and phase of a complex quantity estimated at x₁ = 0.01, x₂ = 0; GUM and Monte Carlo results compared (Table 6).


#### Step 1. What do we measure, and how is it calculated?

This project has 2 results (output quantities) that are calculated together from the same inputs. Each has its own equation:

- **R**: `R = sqrt(X1^2 + X2^2)`
- **Theta** rad: `Theta = atan2(X2, X1)`


#### Step 2. The input quantities and their uncertainties

The model has 2 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **X1**. Best value 0.01 with standard uncertainty u = 0.01 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **X2**. Best value 0 with standard uncertainty u = 0.01 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It chooses the coverage factor k for a coverage probability of 95.0 %, using the effective degrees of freedom (Welch–Satterthwaite), and gives the **expanded uncertainty** U = k·u_c.

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

Reference: JCGM 102:2011 Table 6, x₁ = 0.01 (MCM with M = 10⁷; tolerances include the Monte Carlo dispersion for 10⁶ trials).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="jcgm102-9.3-polar-x100-correlated"></a>

### Cartesian to polar co-ordinates, x₁ = 0.1, r = 0.9 (JCGM 102 9.3)

`jcgm102-9.3-polar-x100-correlated`

#### What is this project about?

JCGM 102:2011 9.3: magnitude and phase of a complex quantity estimated at x₁ = 0.1, x₂ = 0; GUM and Monte Carlo results compared (Table 7).


#### Step 1. What do we measure, and how is it calculated?

This project has 2 results (output quantities) that are calculated together from the same inputs. Each has its own equation:

- **R**: `R = sqrt(X1^2 + X2^2)`
- **Theta** rad: `Theta = atan2(X2, X1)`


#### Step 2. The input quantities and their uncertainties

The model has 2 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **X1**. Best value 0.1 with standard uncertainty u = 0.01 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **X2**. Best value 0 with standard uncertainty u = 0.01 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).

**Correlated inputs.** Some inputs are not independent, because a common effect (the same standard, the same instrument, the same temperature) acts on both. The correlation coefficient r tells how strongly; the program adds the corresponding term to the combined uncertainty:

- X1 ↔ X2: r = 0.9


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It chooses the coverage factor k for a coverage probability of 95.0 %, using the effective degrees of freedom (Welch–Satterthwaite), and gives the **expanded uncertainty** U = k·u_c.

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

Reference: JCGM 102:2011 Table 7, x₁ = 0.1 (MCM with M = 10⁷; tolerances include the Monte Carlo dispersion for 10⁶ trials).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="jcgm102-9.3-polar-x100"></a>

### Cartesian to polar co-ordinates, x₁ = 0.1 (JCGM 102 9.3)

`jcgm102-9.3-polar-x100`

#### What is this project about?

JCGM 102:2011 9.3: magnitude and phase of a complex quantity estimated at x₁ = 0.1, x₂ = 0; GUM and Monte Carlo results compared (Table 6).


#### Step 1. What do we measure, and how is it calculated?

This project has 2 results (output quantities) that are calculated together from the same inputs. Each has its own equation:

- **R**: `R = sqrt(X1^2 + X2^2)`
- **Theta** rad: `Theta = atan2(X2, X1)`


#### Step 2. The input quantities and their uncertainties

The model has 2 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **X1**. Best value 0.1 with standard uncertainty u = 0.01 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **X2**. Best value 0 with standard uncertainty u = 0.01 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It chooses the coverage factor k for a coverage probability of 95.0 %, using the effective degrees of freedom (Welch–Satterthwaite), and gives the **expanded uncertainty** U = k·u_c.

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

Reference: JCGM 102:2011 Table 6, x₁ = 0.1 (MCM with M = 10⁷; tolerances include the Monte Carlo dispersion for 10⁶ trials).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Resistance and reactance from simultaneous data (Advanced)

Three outputs from correlated indication values; the alternative uses a multivariate t distribution for the few data.

<a id="jcgm102-9.4-resistance-reactance-alternative"></a>

### Resistance, reactance and impedance from simultaneous indication values, t covariance (JCGM 102 9.4)

`jcgm102-9.4-resistance-reactance-alternative`

#### What is this project about?

JCGM 102:2011 9.4 (GUM H.2 extended): three output quantities from correlated averages; GUM framework with the covariance matrix of the multivariate t-distribution and Monte Carlo method with the multivariate t-distribution.


#### Step 1. What do we measure, and how is it calculated?

This project has 3 results (output quantities) that are calculated together from the same inputs. Each has its own equation:

- **R** Ω: `R = V/(I*1e-3)*cos(phi)`
- **X** Ω: `X = V/(I*1e-3)*sin(phi)`
- **Z** Ω: `Z = V/(I*1e-3)`


#### Step 2. The input quantities and their uncertainties

The model has 0 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:



#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It chooses the coverage factor k for a coverage probability of 95.0 %, using the effective degrees of freedom (Welch–Satterthwaite), and gives the **expanded uncertainty** U = k·u_c.

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

Reference: JCGM 102:2011 Table 11, rows Alt. GUF and MCM (M = 10⁶).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="jcgm102-9.4-resistance-reactance"></a>

### Resistance, reactance and impedance from simultaneous indication values (JCGM 102 9.4)

`jcgm102-9.4-resistance-reactance`

#### What is this project about?

JCGM 102:2011 9.4 (GUM H.2 extended): three output quantities from correlated averages; GUM framework with U_x = M/(n(n − 1)) and Monte Carlo method with the multivariate t-distribution.


#### Step 1. What do we measure, and how is it calculated?

This project has 3 results (output quantities) that are calculated together from the same inputs. Each has its own equation:

- **R** Ω: `R = V/(I*1e-3)*cos(phi)`
- **X** Ω: `X = V/(I*1e-3)*sin(phi)`
- **Z** Ω: `Z = V/(I*1e-3)`


#### Step 2. The input quantities and their uncertainties

The model has 0 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:



#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It chooses the coverage factor k for a coverage probability of 95.0 %, using the effective degrees of freedom (Welch–Satterthwaite), and gives the **expanded uncertainty** U = k·u_c.

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

Reference: JCGM 102:2011 Table 11, rows GUF and MCM (M = 10⁶).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Implicit models: a resistance thermometer (Advanced)

The temperature cannot be written as a formula of the resistance, so the equation is solved numerically. With ten temperatures from one thermometer, the results are strongly correlated.

<a id="jcgm102-9.5.2-resistance-thermometer"></a>

### Celsius temperature from a resistance thermometer, implicit model (JCGM 102 9.5.2)

`jcgm102-9.5.2-resistance-thermometer`

#### What is this project about?

JCGM 102:2011 9.5.2: one temperature from a measured resistance ratio, with the correlated calibration constants of the thermometer.


#### Step 1. What do we measure, and how is it calculated?

This project has 1 results (output quantities) that are calculated together from the same inputs. Each has its own equation:

- **theta** °C: `theta = R0*(1 + A*theta + B*theta^2) - r*RS`

The model is *implicit*: the equations are not solved for the results, so the program solves them numerically.


#### Step 2. The input quantities and their uncertainties

The model has 5 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **R0**. Best value 99.9961 Ω with standard uncertainty u = 0.0005 Ω (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **A**. Best value 0.0039096 °C⁻¹ with standard uncertainty u = 0.0000027 °C⁻¹ (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **B**. Best value -6e-7 °C⁻² with standard uncertainty u = 1.1e-7 °C⁻² (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **RS**. Best value 99.99947 Ω with standard uncertainty u = 0.0001 Ω (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
5. **r**. Best value 1.0780057 with standard uncertainty u = 0.000005 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).

**Correlated inputs.** Some inputs are not independent, because a common effect (the same standard, the same instrument, the same temperature) acts on both. The correlation coefficient r tells how strongly; the program adds the corresponding term to the combined uncertainty:

- R0 ↔ A: r = -0.155
- R0 ↔ B: r = 0.092
- A ↔ B: r = -0.959


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It chooses the coverage factor k for a coverage probability of 95.0 %, using the effective degrees of freedom (Welch–Satterthwaite), and gives the **expanded uncertainty** U = k·u_c.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

Reference: JCGM 102:2011 9.5.2.5 and 9.5.2.6 (θ = 20.023 2 °C, u = 0.004 5 °C).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="jcgm102-9.5.3-ten-temperatures"></a>

### Ten Celsius temperatures with one resistance thermometer, implicit model (JCGM 102 9.5.3)

`jcgm102-9.5.3-ten-temperatures`

#### What is this project about?

JCGM 102:2011 9.5.3: ten temperatures measured with the same calibrated thermometer are correlated through its calibration constants (Tables 15 and 16).


#### Step 1. What do we measure, and how is it calculated?

This project has 10 results (output quantities) that are calculated together from the same inputs. Each has its own equation:

- **theta1** °C: `theta1 = R0*(1 + A*theta1 + B*theta1^2) - r1*RS`
- **theta2** °C: `theta2 = R0*(1 + A*theta2 + B*theta2^2) - r2*RS`
- **theta3** °C: `theta3 = R0*(1 + A*theta3 + B*theta3^2) - r3*RS`
- **theta4** °C: `theta4 = R0*(1 + A*theta4 + B*theta4^2) - r4*RS`
- **theta5** °C: `theta5 = R0*(1 + A*theta5 + B*theta5^2) - r5*RS`
- **theta6** °C: `theta6 = R0*(1 + A*theta6 + B*theta6^2) - r6*RS`
- **theta7** °C: `theta7 = R0*(1 + A*theta7 + B*theta7^2) - r7*RS`
- **theta8** °C: `theta8 = R0*(1 + A*theta8 + B*theta8^2) - r8*RS`
- **theta9** °C: `theta9 = R0*(1 + A*theta9 + B*theta9^2) - r9*RS`
- **theta10** °C: `theta10 = R0*(1 + A*theta10 + B*theta10^2) - r10*RS`

The model is *implicit*: the equations are not solved for the results, so the program solves them numerically.


#### Step 2. The input quantities and their uncertainties

The model has 14 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **R0**. Best value 99.9961 Ω with standard uncertainty u = 0.0005 Ω (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **A**. Best value 0.0039096 °C⁻¹ with standard uncertainty u = 0.0000027 °C⁻¹ (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **B**. Best value -6e-7 °C⁻² with standard uncertainty u = 1.1e-7 °C⁻² (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **RS**. Best value 99.99947 Ω with standard uncertainty u = 0.0001 Ω (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
5. **r1**. Best value 1.0000053 with standard uncertainty u = 0.000005 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
6. **r2**. Best value 1.0150054 with standard uncertainty u = 0.000005 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
7. **r3**. Best value 1.0300055 with standard uncertainty u = 0.000005 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
8. **r4**. Best value 1.0450056 with standard uncertainty u = 0.000005 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
9. **r5**. Best value 1.0600056 with standard uncertainty u = 0.000005 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
10. **r6**. Best value 1.0780057 with standard uncertainty u = 0.000005 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
11. **r7**. Best value 1.0900058 with standard uncertainty u = 0.000005 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
12. **r8**. Best value 1.1050059 with standard uncertainty u = 0.000005 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
13. **r9**. Best value 1.120006 with standard uncertainty u = 0.000005 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
14. **r10**. Best value 1.0780057 with standard uncertainty u = 0.000005 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).

**Correlated inputs.** Some inputs are not independent, because a common effect (the same standard, the same instrument, the same temperature) acts on both. The correlation coefficient r tells how strongly; the program adds the corresponding term to the combined uncertainty:

- R0 ↔ A: r = -0.155
- R0 ↔ B: r = 0.092
- A ↔ B: r = -0.959


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It chooses the coverage factor k for a coverage probability of 95.0 %, using the effective degrees of freedom (Welch–Satterthwaite), and gives the **expanded uncertainty** U = k·u_c.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

Reference: JCGM 102:2011 Tables 15 and 16.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Peak areas from the same injections (Advanced)

Benzo[a]pyrene by GC-MS with an internal standard. Two areas measured in the same injections are correlated; the variant treats them as an ensemble when counting degrees of freedom.

<a id="gum5-e2-benzo-a-pyrene-ensemble"></a>

### Benzo[a]pyrene: peak areas of the same injections as an ensemble in ν_eff (JCGM GUM-5 Example 2, Willink 2007)

`gum5-e2-benzo-a-pyrene-ensemble`

#### What is this project about?

The same budget with A_E and A_ISE declared an ensemble: their joint variance is one Welch–Satterthwaite component with ν = 2, giving ν_eff ≈ 2.1 instead of 5.2 or 4.8.


#### Step 1. What do we measure, and how is it calculated?

The result is **mE** in ng (Mass of benzo[a]pyrene in 1 µL of extract). It is not measured directly: it is calculated from the input quantities with the measurement model

```
mE = f*AE/AISE*mISE
```


#### Step 2. The input quantities and their uncertainties

The model has 4 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **f**: GC-MS calibration factor (mean of three factors, ν = 9). Best value 0.616 with standard uncertainty u = 0.017 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings). This standard uncertainty is itself only known roughly: it has 9 degrees of freedom.
2. **mISE**: Mass of internal standard BaP-d12 in 1 µL of extract (certificate). Best value 0.2455 ng with standard uncertainty u = 0.0036 ng (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **AE**: Mean BaP peak area of three injections. Best value 85114 a.u. with standard uncertainty u = 9564 a.u. (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings). This standard uncertainty is itself only known roughly: it has 2 degrees of freedom.
4. **AISE**: Mean BaP-d12 peak area of the same three injections. Best value 917546 a.u. with standard uncertainty u = 44492 a.u. (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings). This standard uncertainty is itself only known roughly: it has 2 degrees of freedom.

**Correlated inputs.** Some inputs are not independent, because a common effect (the same standard, the same instrument, the same temperature) acts on both. The correlation coefficient r tells how strongly; the program adds the corresponding term to the combined uncertainty:

- f ↔ mISE: r = -0.5392
- AE ↔ AISE: r = -0.4771


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 0.014 ng
- standard uncertainty u = 0.00202 ng

Reference: Budget of JCGM GUM-5:2026 Example 2 (u = 0.002 ng); ν_eff = 2.11 recomputed with the ensemble rule of Willink (2007), not printed in GUM-5.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="gum5-e2-benzo-a-pyrene"></a>

### Benzo[a]pyrene by GC-MS with an internal standard, correlated inputs (JCGM GUM-5 Example 2)

`gum5-e2-benzo-a-pyrene`

#### What is this project about?

Internal-standard model with two correlated pairs of inputs; LPU with covariances and the Welch–Satterthwaite ν_eff as printed.


#### Step 1. What do we measure, and how is it calculated?

The result is **mE** in ng (Mass of benzo[a]pyrene in 1 µL of extract). It is not measured directly: it is calculated from the input quantities with the measurement model

```
mE = f*AE/AISE*mISE
```


#### Step 2. The input quantities and their uncertainties

The model has 4 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **f**: GC-MS calibration factor (mean of three factors, ν = 9). Best value 0.616 with standard uncertainty u = 0.017 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings). This standard uncertainty is itself only known roughly: it has 9 degrees of freedom.
2. **mISE**: Mass of internal standard BaP-d12 in 1 µL of extract (certificate). Best value 0.2455 ng with standard uncertainty u = 0.0036 ng (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **AE**: Mean BaP peak area of three injections. Best value 85114 a.u. with standard uncertainty u = 9564 a.u. (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings). This standard uncertainty is itself only known roughly: it has 2 degrees of freedom.
4. **AISE**: Mean BaP-d12 peak area of the same three injections. Best value 917546 a.u. with standard uncertainty u = 44492 a.u. (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings). This standard uncertainty is itself only known roughly: it has 2 degrees of freedom.

**Correlated inputs.** Some inputs are not independent, because a common effect (the same standard, the same instrument, the same temperature) acts on both. The correlation coefficient r tells how strongly; the program adds the corresponding term to the combined uncertainty:

- f ↔ mISE: r = -0.5392
- AE ↔ AISE: r = -0.4771


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 0.014 ng
- standard uncertainty u = 0.00202 ng
- coverage factor k = 2.5

Reference: JCGM GUM-5:2026, 2.4.1.2–2.4.1.4 and Table 2.3: m_E = 0.014 ng, u = 0.002 ng (u² = 4.1·10⁻⁶ ng²), ν_eff = 5.2, k = 2.5.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## A gravimetric gas mixture (Advanced)

Eight amount fractions of a calibration gas mixture prepared by weighing: many inputs, intermediate quantities and correlated outputs.

<a id="gum5-e4-gravimetric-gas-mixture"></a>

### Gravimetric preparation of a CO₂ in N₂ calibration gas mixture: amount fractions of eight components and their correlations (JCGM GUM-5 Example 4)

`gum5-e4-gravimetric-gas-mixture`

#### What is this project about?

ISO 6142-1 model y_k = n_k/n_tot from three weighings, the purity tables of two parent gases and IUPAC atomic-weight intervals; multivariate LPU (JCGM 102).


#### Step 1. What do we measure, and how is it calculated?

This project has 8 results (output quantities) that are calculated together from the same inputs. Each has its own equation:

- **y_Ar** µmol/mol: `y_Ar = 1e6*(m2*1e-6*x2_Ar/MP2)/ntot`
- **y_CH4** µmol/mol: `y_CH4 = 1e6*(m1*1e-6*x1_CH4/MP1 + m2*1e-6*x2_CH4/MP2)/ntot`
- **y_CO** µmol/mol: `y_CO = 1e6*(m1*1e-6*x1_CO/MP1 + m2*1e-6*x2_CO/MP2)/ntot`
- **y_CO2** µmol/mol: `y_CO2 = 1e6*(m1*x1_CO2/MP1 + m2*1e-6*x2_CO2/MP2)/ntot`
- **y_H2** µmol/mol: `y_H2 = 1e6*(m2*1e-6*x2_H2/MP2)/ntot`
- **y_H2O** µmol/mol: `y_H2O = 1e6*(m1*1e-6*x1_H2O/MP1 + m2*1e-6*x2_H2O/MP2)/ntot`
- **y_N2** µmol/mol: `y_N2 = 1e6*(m1*1e-6*x1_N2/MP1 + m2*x2_N2/MP2)/ntot`
- **y_O2** µmol/mol: `y_O2 = 1e6*(m1*1e-6*x1_O2/MP1 + m2*1e-6*x2_O2/MP2)/ntot`

To keep the equation readable, 15 intermediate quantities are calculated first:

- `M_Ar = A_Ar`
- `M_CH4 = 4*A_H + A_C`
- `M_CO = A_C + A_O`
- `M_CO2 = A_C + 2*A_O`
- `M_H2 = 2*A_H`
- `M_H2O = 2*A_H + A_O`
- `M_N2 = 2*A_N`
- `M_O2 = 2*A_O`
- `x1_CO2 = 1 - 1e-6*(x1_CH4 + x1_CO + x1_H2O + x1_N2 + x1_O2)`
- `x2_N2 = 1 - 1e-6*(x2_Ar + x2_CH4 + x2_CO + x2_CO2 + x2_H2 + x2_H2O + x2_O2)`
- `MP1 = 1e-6*x1_CH4*M_CH4 + 1e-6*x1_CO*M_CO + x1_CO2*M_CO2 + 1e-6*x1_H2O*M_H2O + 1e-6*x1_N2*M_N2 + 1e-6*x1_O2*M_O2`
- `MP2 = 1e-6*x2_Ar*M_Ar + 1e-6*x2_CH4*M_CH4 + 1e-6*x2_CO*M_CO + 1e-6*x2_CO2*M_CO2 + 1e-6*x2_H2*M_H2 + 1e-6*x2_H2O*M_H2O + x2_N2*M_N2 + 1e-6*x2_O2*M_O2`
- `m1 = r1 - r0`
- `m2 = r2 - r1`
- `ntot = m1/MP1 + m2/MP2`


#### Step 2. The input quantities and their uncertainties

The model has 20 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **r0**: Mean balance difference, sample − reference cylinder, evacuated. Best value -152.0243 g with standard uncertainty u = 0.0005 g (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **r1**: Mean balance difference, sample − reference cylinder, after parent gas 1. Best value 93.4951 g with standard uncertainty u = 0.0008 g (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **r2**: Mean balance difference, sample − reference cylinder, after parent gas 2. Best value 717.274 g with standard uncertainty u = 0.0007 g (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **A_H**: Standard atomic weight of H (IUPAC interval). Only limits are known: the value lies between 1.00784 and 1.00811 (1.007975 ± 0.00013500), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.00013500/√3 = 7.79 × 10⁻⁵.
5. **A_C**: Standard atomic weight of C (IUPAC interval). Only limits are known: the value lies between 12.0096 and 12.0116 (12.0106 ± 0.0010000), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.0010000/√3 = 0.000577.
6. **A_N**: Standard atomic weight of N (IUPAC interval). Only limits are known: the value lies between 14.00643 and 14.00728 (14.006855 ± 0.00042500), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.00042500/√3 = 0.000245.
7. **A_O**: Standard atomic weight of O (IUPAC interval). Only limits are known: the value lies between 15.99903 and 15.99977 (15.9994 ± 0.00037000), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.00037000/√3 = 0.000214.
8. **A_Ar**: Standard atomic weight of Ar (IUPAC interval). Only limits are known: the value lies between 39.792 and 39.963 (39.8775 ± 0.085500), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = 0.085500/√3 = 0.0494.
9. **x1_CH4**: CH4 in the carbon dioxide parent gas. Best value 0.1 µmol/mol with standard uncertainty u = 0.06 µmol/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
10. **x1_CO**: CO in the carbon dioxide parent gas. Best value 0.5 µmol/mol with standard uncertainty u = 0.3 µmol/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
11. **x1_H2O**: H2O in the carbon dioxide parent gas. Best value 1.9 µmol/mol with standard uncertainty u = 0.1 µmol/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
12. **x1_N2**: N2 in the carbon dioxide parent gas. Best value 4.3 µmol/mol with standard uncertainty u = 2.1 µmol/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
13. **x1_O2**: O2 in the carbon dioxide parent gas. Best value 5.3 µmol/mol with standard uncertainty u = 2.6 µmol/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
14. **x2_Ar**: Ar in the nitrogen parent gas. Best value 5 µmol/mol with standard uncertainty u = 3 µmol/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
15. **x2_CH4**: CH4 in the nitrogen parent gas. Best value 0.008 µmol/mol with standard uncertainty u = 0.005 µmol/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
16. **x2_CO**: CO in the nitrogen parent gas. Best value 0.015 µmol/mol with standard uncertainty u = 0.009 µmol/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
17. **x2_CO2**: CO2 in the nitrogen parent gas. Best value 0.01 µmol/mol with standard uncertainty u = 0.006 µmol/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
18. **x2_H2**: H2 in the nitrogen parent gas. Best value 0.025 µmol/mol with standard uncertainty u = 0.015 µmol/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
19. **x2_H2O**: H2O in the nitrogen parent gas. Best value 0.01 µmol/mol with standard uncertainty u = 0.006 µmol/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
20. **x2_O2**: O2 in the nitrogen parent gas. Best value 0.005 µmol/mol with standard uncertainty u = 0.003 µmol/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

Reference: JCGM GUM-5:2026 Table 4.6 and 4.3.6.9; expected values recomputed: Ar 4.0 ± 2.4, CH₄ 0.026 ± 0.013, CO 0.112 ± 0.061, H₂ 0.020 ± 0.012, O₂ 1.07 ± 0.52, N₂ 799 649 ± 5 µmol/mol as printed; CO₂ 200 345 ± 4 (printed ± 5), H₂O 0.389 (printed 0.379); r(CO₂, N₂) −0.85 (−0.86), r(Ar, N₂) −0.55 (−0.50).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Calibrating a torque sensor (Advanced)

A straight line through the origin fitted by ordinary and weighted least squares, with the GUM uncertainty of the slope.

<a id="gum5-e9-torque-slope-ols"></a>

### Torque sensor calibration, straight line through the origin, ordinary least squares with the GUF (JCGM GUM-5 Example 9)

`gum5-e9-torque-slope-ols`

#### What is this project about?

Slope β of Y = βX from eight torque levels; the OLS estimator (9.4) is linear in the means Y_i, so the LPU and Welch–Satterthwaite apply directly.


#### Step 1. What do we measure, and how is it calculated?

The result is **beta** (Slope of the calibration line Y = βX). It is not measured directly: it is calculated from the input quantities with the measurement model

```
beta = 0.002593756963*Y1 + 0.005161833164*Y2 + 0.007832632413*Y3 + 0.01286606177*Y4 + 0.02570644277*Y5 + 0.07704228603*Y6 + 0.1027487288*Y7 + 0.06429178769*Y8
```


#### Step 2. The input quantities and their uncertainties

The model has 8 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **Y1**: Mean sensor response at x = 0.101 N·m (6 readings, s = 0.0055 N·m). Best value 0.095 N·m with standard uncertainty u = 0.0022454 N·m (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings). This standard uncertainty is itself only known roughly: it has 5 degrees of freedom.
2. **Y2**: Mean sensor response at x = 0.201 N·m (6 readings, s = 0.0052 N·m). Best value 0.1966 N·m with standard uncertainty u = 0.0021229 N·m (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings). This standard uncertainty is itself only known roughly: it has 5 degrees of freedom.
3. **Y3**: Mean sensor response at x = 0.305 N·m (6 readings, s = 0.0041 N·m). Best value 0.3016 N·m with standard uncertainty u = 0.0016738 N·m (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings). This standard uncertainty is itself only known roughly: it has 5 degrees of freedom.
4. **Y4**: Mean sensor response at x = 0.501 N·m (6 readings, s = 0.0041 N·m). Best value 0.4983 N·m with standard uncertainty u = 0.0016738 N·m (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings). This standard uncertainty is itself only known roughly: it has 5 degrees of freedom.
5. **Y5**: Mean sensor response at x = 1.001 N·m (6 readings, s = 0.0098 N·m). Best value 1.0083 N·m with standard uncertainty u = 0.0040008 N·m (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings). This standard uncertainty is itself only known roughly: it has 5 degrees of freedom.
6. **Y6**: Mean sensor response at x = 3.0 N·m (6 readings, s = 0.0082 N·m). Best value 3.0266 N·m with standard uncertainty u = 0.0033476 N·m (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings). This standard uncertainty is itself only known roughly: it has 5 degrees of freedom.
7. **Y7**: Mean sensor response at x = 4.001 N·m (6 readings, s = 0.0121 N·m). Best value 4.0466 N·m with standard uncertainty u = 0.0049398 N·m (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings). This standard uncertainty is itself only known roughly: it has 5 degrees of freedom.
8. **Y8**: Mean sensor response at x = 5.007 N·m (3 readings, s = 0.0379 N·m). Best value 5.0666 N·m with standard uncertainty u = 0.021882 N·m (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings). This standard uncertainty is itself only known roughly: it has 2 degrees of freedom.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 1.0107
- standard uncertainty u = 0.0015
- expanded uncertainty U = 0.00515

Reference: JCGM GUM-5:2026 Table 9.3, OLS: β = 1.0107, u = 0.0015, [1.0055, 1.0158]; ν_eff = 2.7 in 9.5.1.4.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="gum5-e9-torque-slope-wls"></a>

### Torque sensor calibration, straight line through the origin, weighted least squares with the GUF (JCGM GUM-5 Example 9)

`gum5-e9-torque-slope-wls`

#### What is this project about?

Slope β of Y = βX from eight torque levels; the WLS estimator (9.4) is linear in the means Y_i, so the LPU and Welch–Satterthwaite apply directly.


#### Step 1. What do we measure, and how is it calculated?

The result is **beta** (Slope of the calibration line Y = βX). It is not measured directly: it is calculated from the input quantities with the measurement model

```
beta = 0.01172995665*Y1 + 0.02611498511*Y2 + 0.06374300572*Y3 + 0.1047057242*Y4 + 0.03661696445*Y5 + 0.156745096*Y6 + 0.09600596629*Y7 + 0.006123075128*Y8
```


#### Step 2. The input quantities and their uncertainties

The model has 8 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **Y1**: Mean sensor response at x = 0.101 N·m (6 readings, s = 0.0055 N·m). Best value 0.095 N·m with standard uncertainty u = 0.0022454 N·m (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings). This standard uncertainty is itself only known roughly: it has 5 degrees of freedom.
2. **Y2**: Mean sensor response at x = 0.201 N·m (6 readings, s = 0.0052 N·m). Best value 0.1966 N·m with standard uncertainty u = 0.0021229 N·m (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings). This standard uncertainty is itself only known roughly: it has 5 degrees of freedom.
3. **Y3**: Mean sensor response at x = 0.305 N·m (6 readings, s = 0.0041 N·m). Best value 0.3016 N·m with standard uncertainty u = 0.0016738 N·m (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings). This standard uncertainty is itself only known roughly: it has 5 degrees of freedom.
4. **Y4**: Mean sensor response at x = 0.501 N·m (6 readings, s = 0.0041 N·m). Best value 0.4983 N·m with standard uncertainty u = 0.0016738 N·m (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings). This standard uncertainty is itself only known roughly: it has 5 degrees of freedom.
5. **Y5**: Mean sensor response at x = 1.001 N·m (6 readings, s = 0.0098 N·m). Best value 1.0083 N·m with standard uncertainty u = 0.0040008 N·m (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings). This standard uncertainty is itself only known roughly: it has 5 degrees of freedom.
6. **Y6**: Mean sensor response at x = 3.0 N·m (6 readings, s = 0.0082 N·m). Best value 3.0266 N·m with standard uncertainty u = 0.0033476 N·m (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings). This standard uncertainty is itself only known roughly: it has 5 degrees of freedom.
7. **Y7**: Mean sensor response at x = 4.001 N·m (6 readings, s = 0.0121 N·m). Best value 4.0466 N·m with standard uncertainty u = 0.0049398 N·m (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings). This standard uncertainty is itself only known roughly: it has 5 degrees of freedom.
8. **Y8**: Mean sensor response at x = 5.007 N·m (3 readings, s = 0.0379 N·m). Best value 5.0666 N·m with standard uncertainty u = 0.021882 N·m (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings). This standard uncertainty is itself only known roughly: it has 2 degrees of freedom.


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 1.0085
- standard uncertainty u = 0.0008
- expanded uncertainty U = 0.0016

Reference: JCGM GUM-5:2026 Table 9.3, WLS: β = 1.0085, u = 0.0008, [1.0069, 1.0101]; ν_eff = 14 in 9.5.1.4; recomputed 13.31 (F83).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Between-bottle homogeneity with Bayes (Advanced)

When the between-bottle variance is small, the classical estimate can be zero or negative; a hierarchical Bayesian model gives a sensible value.

<a id="gum5-e12-between-bottle-homogeneity-bayes"></a>

### Between-bottle homogeneity of nitrogen in natural gas mixtures: ANOVA, u*_bb and a Bayesian hierarchical model (JCGM GUM-5 Example 12)

`gum5-e12-between-bottle-homogeneity-bayes`

#### What is this project about?

Ten gas mixtures measured five times each; MS_between < MS_within, so the ANOVA gives no between-bottle standard deviation. The Bayesian model with half-Cauchy priors gives a posterior for τ.


#### Step 1. The idea: comparing laboratories

**Homogeneity**: several units of the material are measured in replicate. An analysis of variance separates the variation between units from the repeatability of the measurement; the between-unit standard deviation must be small compared with σ_pt.


#### Finally: what the source expects

Reference: JCGM GUM-5:2026, Table 12.2 (MS 3.25·10⁻⁷ and 4.90·10⁻⁷), 12.4.5 (τ 0.00014 ± 0.00011, [0.00001, 0.00040]; σ 0.00069; µ 0.42514), Table 12.3 (HPD) and 12.4.8 (median 1.1 µmol/mol); u*_bb, the median to 10⁻⁶ and the mode 0 are recomputed here.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Homogeneity of a soil reference material (Advanced)

Twenty bottles measured in triplicate, evaluated by the analysis of variance and by a hierarchical Bayesian model.

<a id="possolo-bodnar-2018-chromium-soil-homogeneity-bayes"></a>

### Chromium in a soil reference material, 20 bottles in triplicate: ANOVA and a hierarchical Bayesian model (Possolo & Bodnar 2018; ISO Guide 35 C.1 data)

`possolo-bodnar-2018-chromium-soil-homogeneity-bayes`

#### What is this project about?

w_ij = ω + β_j + ε_ij with bottle effects β_j ~ N(0, τ²) and errors ε_ij ~ N(0, σ²). The value assigned to the material is the estimate of ω; its uncertainty includes the between-bottle and the within-bottle variability.


#### Step 1. The idea: comparing laboratories

**Homogeneity**: several units of the material are measured in replicate. An analysis of variance separates the variation between units from the repeatability of the measurement; the between-unit standard deviation must be small compared with σ_pt.


#### Finally: what the source expects

Reference: Possolo & Bodnar (2018), §4.4 and Table 4: REML 121.62 (0.95) mg/kg, τ 3.93 and σ 2.87 mg/kg (= s_bb and s_r of ISO Guide 35); Bayesian (MCMC) 121.62 (1) mg/kg, Laplace 121.62 (0.98) mg/kg.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Atomic weight from isotope ratios (Advanced)

The atomic weight and isotopic composition of lead from three measured isotope ratios: correlated outputs from a nonlinear model.

<a id="possolo-meija-2022-atomic-weight-lead-hipb1"></a>

### Atomic weight and isotopic composition of lead in NRC HIPB-1 from three isotope ratios (Possolo & Meija 2022)

`possolo-meija-2022-atomic-weight-lead-hipb1`

#### What is this project about?

Tong et al. (2019) measured ²⁰⁴Pb/²⁰⁷Pb, ²⁰⁶Pb/²⁰⁷Pb and ²⁰⁸Pb/²⁰⁷Pb in the high-purity lead HIPB-1. The amount fractions follow from the ratios and the closure Σx = 1; A_r(Pb) is their weighted average of the isotopic masses.


#### Step 1. What do we measure, and how is it calculated?

This project has 5 results (output quantities) that are calculated together from the same inputs. Each has its own equation:

- **x204** mol/mol: `x204 = R4/(1 + R4 + R6 + R8)`
- **x206** mol/mol: `x206 = R6/(1 + R4 + R6 + R8)`
- **x207** mol/mol: `x207 = 1/(1 + R4 + R6 + R8)`
- **x208** mol/mol: `x208 = R8/(1 + R4 + R6 + R8)`
- **ArPb** 1: `ArPb = (A204*R4 + A206*R6 + A207 + A208*R8)/(1 + R4 + R6 + R8)`


#### Step 2. The input quantities and their uncertainties

The model has 7 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **R4**: Isotope ratio n(²⁰⁴Pb)/n(²⁰⁷Pb), MC-ICP-MS (U95 % = 0.000 06, k = 2). Best value 0.063 mol/mol with standard uncertainty u = 0.00003 mol/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **R6**: Isotope ratio n(²⁰⁶Pb)/n(²⁰⁷Pb) (U95 % = 0.0004). Best value 1.3314 mol/mol with standard uncertainty u = 0.0002 mol/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **R8**: Isotope ratio n(²⁰⁸Pb)/n(²⁰⁷Pb) (U95 % = 0.0008). Best value 2.5193 mol/mol with standard uncertainty u = 0.0004 mol/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **A204**: Relative atomic mass of ²⁰⁴Pb (AME 2020). Best value 203.97 1 with standard uncertainty u = 0.0000012 1 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
5. **A206**: Relative atomic mass of ²⁰⁶Pb (AME 2020). Best value 205.97 1 with standard uncertainty u = 0.0000012 1 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
6. **A207**: Relative atomic mass of ²⁰⁷Pb (AME 2020). Best value 206.98 1 with standard uncertainty u = 0.0000012 1 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
7. **A208**: Relative atomic mass of ²⁰⁸Pb (AME 2020). Best value 207.976652 1 with standard uncertainty u = 0.0000012 1 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.

It also uses the **Monte Carlo method**: it draws 200 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 200 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

Reference: Possolo & Meija (2022), p. 29: u(x) = 0.000 006, 0.000 037, 0.000 019, 0.000 045 mol/mol; correlations −0.02, +0.13, +0.18, −0.17, −0.90, −0.58 (Monte Carlo, 5 × 10⁴); A_r(Pb) = 207.1791, U95 % = 0.0002 (u = U/2; tolerance of the correlations: Monte Carlo error).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## A consensus value with heavy tails (Advanced)

Twenty-two inconsistent estimates of the Hubble–Lemaître constant combined with a Bayesian model whose laboratory effects follow a Laplace distribution, which is less influenced by outliers.

<a id="possolo-meija-2022-hubble-lemaitre-constant-laplace"></a>

### Hubble–Lemaître constant from 22 mutually inconsistent estimates: Bayesian consensus with Laplace effects (Possolo & Meija 2022)

`possolo-meija-2022-hubble-lemaitre-constant-laplace`

#### What is this project about?

Recent estimates of H₀ (Planck's excluded) are more dispersed than their uncertainties suggest. The model H₀,j = H₀ + λ_j + ε_j with Laplace experiment effects λ_j (standard deviation τ), as the NIST Decision Tree recommends, and Gaussian errors with the reported u_j blends them into a consensus value.


#### Step 1. The idea: one value from several laboratories

22 laboratories or experiments measured **H₀**, each with its own uncertainty. Often the results disagree by more than their uncertainties allow: there is an unexplained spread between laboratories (*dark uncertainty*). A **consensus value** combines them while accounting for that spread.

Several procedures are available (DerSimonian–Laird, Linear Pool, hierarchical Bayes, maximum likelihood); they make different assumptions and can give different values. The **degrees of equivalence** D_j = x_j − x_ref show how far each laboratory is from the consensus value, with an uncertainty.


> **Try:** Switch procedures in the **Data** tab and compare the consensus values and the number of laboratories whose degree of equivalence is significant.


#### Finally: what the source expects

Reference: Possolo & Meija (2022), p. 150: posterior mean 71.69 (km/s)/Mpc, standard deviation 0.66 (km/s)/Mpc, median of τ 2.03 (km/s)/Mpc (Stan). numpy grid (2000 × 1500): 71.6915, 0.6603, 2.0336.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## An implicit chemical model (Advanced)

The Mohr titration with its endpoint chemistry: the solubility equilibrium gives an implicit equation that the program solves.

<a id="iupac2024-3a-argentometric-titration-ksp"></a>

### Argentometric titration (Mohr) with the endpoint chemistry: implicit model with the solubility products of AgCl and Ag₂CrO₄ (IUPAC 2024, Examples 3A and 4I)

`iupac2024-3a-argentometric-titration-ksp`

#### What is this project about?

At the endpoint [Ag⁺] = √(K_sp2/[CrO₄²⁻]) and [Cl⁻] = K_sp1/[Ag⁺]; the mass balance of silver and chloride gives an equation for c(NaCl) that is solved numerically.


#### Step 1. What do we measure, and how is it calculated?

This project has 1 results (output quantities) that are calculated together from the same inputs. Each has its own equation:

- **c_NaCl** mmol/L: `c_NaCl = 1000*(sqrt(Ksp2*((V_NaCl + dV_NaCl) + (V_Ag + dV_Ag + dV_E))/(c_K*(V_NaCl + dV_NaCl))) - Ksp1/sqrt(Ksp2*((V_NaCl + dV_NaCl) + (V_Ag + dV_Ag + dV_E))/(c_K*(V_NaCl + dV_NaCl)))) - (c_Ag*(V_Ag + dV_Ag + dV_E) - c_NaCl*(V_NaCl + dV_NaCl))/((V_NaCl + dV_NaCl) + (V_Ag + dV_Ag + dV_E))`

The model is *implicit*: the equations are not solved for the results, so the program solves them numerically.


#### Step 2. The input quantities and their uncertainties

The model has 9 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **c_Ag**: Concentration of the AgNO₃ titrant (taken as exact). An exact value, 100 mmol/L, with no uncertainty (for example a defined constant or a nominal value).
2. **V_Ag**: Volume of titrant read at the endpoint. An exact value, 10 mL, with no uncertainty (for example a defined constant or a nominal value).
3. **dV_Ag**: Resolution of the electronic burette, ±0.01 mL. The value lies between -0.01 and 0.01 mL, and values near the centre 0 are more likely than values near the limits (triangular distribution). u = a/√6 = 0.01/√6 = 0.00408 mL.
4. **dV_E**: Visual endpoint detection, one droplet at 95 %. Best value 0 mL with standard uncertainty u = 0.025 mL (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
5. **V_NaCl**: Nominal volume of the sample. An exact value, 100 mL, with no uncertainty (for example a defined constant or a nominal value).
6. **dV_NaCl**: Class A volumetric pipette, triangular with base 0.16 mL. The value lies between -0.08 and 0.08 mL, and values near the centre 0 are more likely than values near the limits (triangular distribution). u = a/√6 = 0.08/√6 = 0.0327 mL.
7. **c_K**: Concentration of the K₂CrO₄ indicator in the sample before titration. An exact value, 0.00175 mol/L, with no uncertainty (for example a defined constant or a nominal value).
8. **Ksp1**: Solubility product of AgCl, lg K = −9.75. An exact value, 1.7783 × 10⁻¹⁰ (mol/L)², with no uncertainty (for example a defined constant or a nominal value).
9. **Ksp2**: Solubility product of Ag₂CrO₄, lg K = −11.9. An exact value, 1.2589 × 10⁻¹² (mol/L)³, with no uncertainty (for example a defined constant or a nominal value).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

Reference: IUPAC 2024, Example 4I (after Meija et al., Eq. 19): relative difference from the conventional estimate 0.24 %; lg K_sp1 = −9.75, lg K_sp2 = −11.9.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Three inconsistent determinations (Advanced)

The molar mass of iridium from three determinations that disagree. Laboratory effects are modelled with a hierarchical Bayesian model.

<a id="iupac2024-3e-4m-iridium-molar-mass-bayes"></a>

### Molar mass of iridium from three mutually inconsistent determinations: laboratory effects and a Bayesian random-effects model (IUPAC 2024, Examples 3E, 4M and 4N)

`iupac2024-3e-4m-iridium-molar-mass-bayes`

#### What is this project about?

Three isotope-ratio determinations behind the 2021 standard atomic weight of iridium (Prohaska et al.) give M = 192.216 77(81), 192.216 62(29) and 192.217 63(17) g/mol. Their standard deviation is 1.6 times the geometric mean of the uncertainties; Cochran's Q test confirms the inconsistency. Laboratory effects with standard deviation τ reconcile them.


#### Step 1. The idea: one value from several laboratories

3 laboratories or experiments measured **M(Ir)**, each with its own uncertainty. Often the results disagree by more than their uncertainties allow: there is an unexplained spread between laboratories (*dark uncertainty*). A **consensus value** combines them while accounting for that spread.

Several procedures are available (DerSimonian–Laird, Linear Pool, hierarchical Bayes, maximum likelihood); they make different assumptions and can give different values. The **degrees of equivalence** D_j = x_j − x_ref show how far each laboratory is from the consensus value, with an uncertainty.


> **Try:** Switch procedures in the **Data** tab and compare the consensus values and the number of laboratories whose degree of equivalence is significant.


#### Finally: what the source expects

Reference: IUPAC 2024, Example 3E: p = 0.0082; Example 4N: M = 192.2171(4) g/mol, median of τ 0.0005 g/mol (MCMC). Grid in numpy: 192.217 119, 0.000 440, τ median 0.000 506.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## A key comparison (Advanced)

CCQM-K145, nickel in bovine liver: a consensus value with dark uncertainty by maximum likelihood. Participants are coded.

<a id="iupac2024-4f-ccqm-k145-nickel-bovine-liver"></a>

### Key comparison CCQM-K145, nickel in bovine liver: consensus value with dark uncertainty by maximum likelihood (IUPAC 2024, Example 4F)

`iupac2024-4f-ccqm-k145-nickel-bovine-liver`

#### What is this project about?

17 national metrology institutes measured the mass fraction of nickel in bovine liver. The results are more dispersed than their stated uncertainties suggest; the laboratory-effects model x_j = μ + λ_j + ε_j with λ_j ~ N(0, τ²) recognises the dark uncertainty τ (the variability between laboratories that the stated uncertainties do not explain).


#### Step 1. The idea: one value from several laboratories

17 laboratories or experiments measured **w(Ni)**, each with its own uncertainty. Often the results disagree by more than their uncertainties allow: there is an unexplained spread between laboratories (*dark uncertainty*). A **consensus value** combines them while accounting for that spread.

Several procedures are available (DerSimonian–Laird, Linear Pool, hierarchical Bayes, maximum likelihood); they make different assumptions and can give different values. The **degrees of equivalence** D_j = x_j − x_ref show how far each laboratory is from the consensus value, with an uncertainty.


> **Try:** Switch procedures in the **Data** tab and compare the consensus values and the number of laboratories whose degree of equivalence is significant.


#### Finally: what the source expects

Reference: IUPAC 2024, Example 4F: μ̂ = 2.043(15) mg/kg and τ̂ = 0.044(14) mg/kg (scipy: 2.042939, 0.015442; 0.043541, 0.014239).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Consensus values of discrepant data (Advanced)

Half-lives and the gravitational constant: measurements that disagree far more than their uncertainties. DerSimonian–Laird, Linear Pool and hierarchical Bayes give different answers; the choice is part of the evaluation.

<a id="koepke2017-half-life-cs137"></a>

### Half-life of ¹³⁷Cs: consensus value of 19 discrepant measurements (DerSimonian–Laird and Linear Pool; Koepke et al. 2017)

`koepke2017-half-life-cs137`

#### What is this project about?

The 19 published half-lives compiled by MacMahon et al. (2004) are heterogeneous (Q = 336 with 18 degrees of freedom). The consensus value is computed in days, as the data are given; Koepke et al. report it in years of 365.25 d.


#### Step 1. The idea: one value from several laboratories

19 laboratories or experiments measured **T½(¹³⁷Cs)**, each with its own uncertainty. Often the results disagree by more than their uncertainties allow: there is an unexplained spread between laboratories (*dark uncertainty*). A **consensus value** combines them while accounting for that spread.

Several procedures are available (DerSimonian–Laird, Linear Pool, hierarchical Bayes, maximum likelihood); they make different assumptions and can give different values. The **degrees of equivalence** D_j = x_j − x_ref show how far each laboratory is from the consensus value, with an uncertainty.


> **Try:** Switch procedures in the **Data** tab and compare the consensus values and the number of laboratories whose degree of equivalence is significant.


#### Finally: what the source expects

Reference: Koepke et al. (2017) Table 4: DerSimonian–Laird 30.04 a, u 0.04 a; Linear Pool 29.94 a, u 0.93 a; hierarchical Bayesian 29.94 a, u 0.19 a (× 365.25 d, tolerance half a unit in the last printed digit, plus the Monte Carlo error of 10⁵ draws for the Linear Pool). τ, Q, the bootstrap u and percentiles and the Linear Pool percentiles: independent numpy computation.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="koepke2017-half-life-sr90"></a>

### Half-life of ⁹⁰Sr: consensus value of 11 discrepant measurements (DerSimonian–Laird and Linear Pool; Koepke et al. 2017)

`koepke2017-half-life-sr90`

#### What is this project about?

The 11 published half-lives compiled by MacMahon et al. (2004) are heterogeneous (Q = 400 with 10 degrees of freedom). The consensus value is computed in days, as the data are given; Koepke et al. report it in years of 365.25 d.


#### Step 1. The idea: one value from several laboratories

11 laboratories or experiments measured **T½(⁹⁰Sr)**, each with its own uncertainty. Often the results disagree by more than their uncertainties allow: there is an unexplained spread between laboratories (*dark uncertainty*). A **consensus value** combines them while accounting for that spread.

Several procedures are available (DerSimonian–Laird, Linear Pool, hierarchical Bayes, maximum likelihood); they make different assumptions and can give different values. The **degrees of equivalence** D_j = x_j − x_ref show how far each laboratory is from the consensus value, with an uncertainty.


> **Try:** Switch procedures in the **Data** tab and compare the consensus values and the number of laboratories whose degree of equivalence is significant.


#### Finally: what the source expects

Reference: Koepke et al. (2017) Table 4: DerSimonian–Laird 28.73 a, u 0.11 a; Linear Pool 28.68 a, u 0.77 a; hierarchical Bayesian 28.71 a, u 0.16 a (× 365.25 d, tolerance half a unit in the last printed digit, plus the Monte Carlo error of 10⁵ draws for the Linear Pool). τ, Q, the bootstrap u and percentiles and the Linear Pool percentiles: independent numpy computation.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="koepke2017-newtonian-constant-g"></a>

### Newtonian constant of gravitation: consensus value of 14 measurements (DerSimonian–Laird and Linear Pool; Koepke et al. 2017)

`koepke2017-newtonian-constant-g`

#### What is this project about?

The 14 measurements of G used in the CODATA 2014 adjustment are mutually inconsistent (Cochran's Q = 319 with 13 degrees of freedom): the measured values are about four times more dispersed than the typical stated uncertainty. The random-effects model recognises this dark uncertainty τ.


#### Step 1. The idea: one value from several laboratories

14 laboratories or experiments measured **G**, each with its own uncertainty. Often the results disagree by more than their uncertainties allow: there is an unexplained spread between laboratories (*dark uncertainty*). A **consensus value** combines them while accounting for that spread.

Several procedures are available (DerSimonian–Laird, Linear Pool, hierarchical Bayes, maximum likelihood); they make different assumptions and can give different values. The **degrees of equivalence** D_j = x_j − x_ref show how far each laboratory is from the consensus value, with an uncertainty.


> **Try:** Switch procedures in the **Data** tab and compare the consensus values and the number of laboratories whose degree of equivalence is significant.


#### Finally: what the source expects

Reference: Koepke et al. (2017) Table 2: DerSimonian–Laird 6.673 80, u 0.000 28, (6.673 20, 6.674 40); Linear Pool 6.673 67, 0.001 24, (6.671 18, 6.675 79); maximum likelihood 6.673 78, 0.000 30; hierarchical Bayesian 6.673 77, 0.000 33, (6.673 10, 6.674 42). τ, Q, u_KH, the bootstrap u and percentiles and D_j: independent numpy computation (10⁵ replicates; the printed interval is consistent with them rounded to 10⁻⁴).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Shades of dark uncertainty (Advanced)

Sixteen measurements of the Newtonian constant of gravitation evaluated with several procedures; the consensus value and its uncertainty depend strongly on the procedure.

<a id="merkatas2019-newtonian-constant-g-shades"></a>

### Newtonian constant of gravitation from 16 measurements: DerSimonian–Laird with Knapp–Hartung, maximum likelihood, hierarchical Bayes and Linear Pool (Merkatas et al. 2019)

`merkatas2019-newtonian-constant-g-shades`

#### What is this project about?

The data of Koepke et al. (2017) with JILA-10 corrected (6.672 60 ± 0.000 25) and the two HUST results of 2018 (time-of-swing and angular-acceleration-feedback). Merkatas et al. compare fourteen procedures; four of them are implemented in the program.


#### Step 1. The idea: one value from several laboratories

16 laboratories or experiments measured **G**, each with its own uncertainty. Often the results disagree by more than their uncertainties allow: there is an unexplained spread between laboratories (*dark uncertainty*). A **consensus value** combines them while accounting for that spread.

Several procedures are available (DerSimonian–Laird, Linear Pool, hierarchical Bayes, maximum likelihood); they make different assumptions and can give different values. The **degrees of equivalence** D_j = x_j − x_ref show how far each laboratory is from the consensus value, with an uncertainty.


> **Try:** Switch procedures in the **Data** tab and compare the consensus values and the number of laboratories whose degree of equivalence is significant.


#### Finally: what the source expects

Reference: Merkatas et al. (2019), Table 4, rows DL, ML, BG and LOP. ML u from the paper's table; BG and LOP from MCMC and Monte Carlo (tolerances: half a unit plus the sampling error of MCMC, about 3 × 10⁻⁶ for the mean and 10⁻⁵ for the limits; exact grid 6.673 896, 0.000 271, (6.673 342, 6.674 415), median τ 0.000 936).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Degrees of equivalence in a comparison (Advanced)

The examples of the NIST Consensus Builder: a consensus value from laboratories with degrees of freedom, and leave-one-out and bilateral degrees of equivalence of 19 laboratories.

<a id="nicob-pcb28-sediment-degrees-of-freedom"></a>

### PCB 28 in sediment from six laboratories with degrees of freedom: DerSimonian–Laird, Linear Pool and leave-one-out degrees of equivalence (NIST Consensus Builder §4.2)

`nicob-pcb28-sediment-degrees-of-freedom`

#### What is this project about?

Mass fraction of PCB 28 (2,4,4'-trichlorobiphenyl) in a sediment, measured by six laboratories that state the degrees of freedom of their standard uncertainties (Exhibit 6). One ν is 2: its Student t distribution has no finite variance, and NICOB rescales it with an approximation (DoEUnilateralDerSimonianLaird.R, linearOP.R), which the program follows.


#### Step 1. The idea: one value from several laboratories

6 laboratories or experiments measured **w(PCB 28)**, each with its own uncertainty. Often the results disagree by more than their uncertainties allow: there is an unexplained spread between laboratories (*dark uncertainty*). A **consensus value** combines them while accounting for that spread.

Several procedures are available (DerSimonian–Laird, Linear Pool, hierarchical Bayes, maximum likelihood); they make different assumptions and can give different values. The **degrees of equivalence** D_j = x_j − x_ref show how far each laboratory is from the consensus value, with an uncertainty.


> **Try:** Switch procedures in the **Data** tab and compare the consensus values and the number of laboratories whose degree of equivalence is significant.


#### Finally: what the source expects

Reference: NICOB manual Exhibit 7: DerSimonian–Laird 33.6 ng/g, u 0.77, (32.0, 35.2); Linear Pool 33.6, 1.55, (31.4, 36.3) ng/g (tolerances: printed digit and Monte Carlo dispersion, see the notes). Degrees of equivalence: numpy port of the NICOB source, 10⁶ draws; tolerance 3 % of U95 %.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="nicob-co60-activity-leave-one-out-bilateral"></a>

### Activity of ⁶⁰Co in 19 laboratories: consensus value with leave-one-out and bilateral degrees of equivalence (NIST Consensus Builder §4.4)

`nicob-co60-activity-leave-one-out-bilateral`

#### What is this project about?

SIR equivalent activities of ⁶⁰Co from the ongoing key comparison BIPM.RI(II)-K1.Co-60, as listed in the NICOB manual (Exhibit 19). Cochran's Q = 36.9 with 18 degrees of freedom (p = 0.005): the results are more dispersed than their uncertainties, and the dark uncertainty is τ̂ = 11.9 kBq.


#### Step 1. The idea: one value from several laboratories

19 laboratories or experiments measured **A (SIR equivalent activity)**, each with its own uncertainty. Often the results disagree by more than their uncertainties allow: there is an unexplained spread between laboratories (*dark uncertainty*). A **consensus value** combines them while accounting for that spread.

Several procedures are available (DerSimonian–Laird, Linear Pool, hierarchical Bayes, maximum likelihood); they make different assumptions and can give different values. The **degrees of equivalence** D_j = x_j − x_ref show how far each laboratory is from the consensus value, with an uncertainty.


> **Try:** Switch procedures in the **Data** tab and compare the consensus values and the number of laboratories whose degree of equivalence is significant.


#### Finally: what the source expects

Reference: NICOB manual Exhibit 21: DerSimonian–Laird 7062 kBq, u 4 kBq, (7053, 7071) kBq; Linear Pool 7064, 29, (7012, 7127) kBq (tolerances: the printed digit and the Monte Carlo dispersion). Degrees of equivalence: numpy port of the NICOB source, 10⁶ draws; tolerance 3 % of U95 % (5σ of the program's 10⁵ draws).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Intermediate precision and nested designs (Advanced)

Precision when time and operator change, and the staggered-nested design that separates several factors with few measurements.

<a id="iso5725-3-j1-carbon-steel"></a>

### Carbon in steel: [time + operator]-different intermediate precision (ISO 5725-3 J.1)

`iso5725-3-j1-carbon-steel`

#### What is this project about?

ISO 5725-3:2023 J.1: one routine sample a day re-analysed the next day by another analyst, 29 pairs in a month; s_I(TO) from the pairs, Formula (5).


#### Step 1. The idea: uncertainty from the laboratory's own data

This is a **top-down** evaluation of **w(C)** (% m/m). Instead of modelling every step of the method, it uses data that the laboratory already has: how much the results scatter when the method is repeated on different days (precision) and how far they are, on average, from the true value (bias or recovery). Together these cover most sources of uncertainty at once.

The uncertainty is evaluated at 1 concentration levels: 0.1 % m/m.


#### Step 2. The data

**Precision** (random effects), 1 entry:

- **s_I(TO)**: 27 duplicate pairs; the standard deviation follows from the differences within the pairs.


#### Step 3. What the program calculates

It combines the components as u = √(u(precision)² + u(bias)² + …), at each level, and multiplies by the coverage factor (k = 2 for about 95 %) to get the expanded uncertainty U. The results show u and U, absolute and relative, at every level.


> **Try:** Compare the size of the precision and bias components in the results. If bias dominates, a better reference material or a correction helps more than repeating measurements.


#### Finally: what the source expects

Reference: ISO 5725-3:2023 J.1.2 (s_I(TO) = 2.87 × 10⁻³ %).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="iso5725-3-j2-vanadium-steel"></a>

### Vanadium in steel: staggered-nested design, level 1 (ISO 5725-3 J.2)

`iso5725-3-j2-vanadium-steel`

#### What is this project about?

ISO 5725-3:2023 J.2: 20 laboratories, two results on one day and one on the next; analysis of variance gives s_r, the time-different intermediate precision s_I(T) and s_R.


#### Step 1. The idea: uncertainty from the laboratory's own data

This is a **top-down** evaluation of **w(V)** (% m/m). Instead of modelling every step of the method, it uses data that the laboratory already has: how much the results scatter when the method is repeated on different days (precision) and how far they are, on average, from the true value (bias or recovery). Together these cover most sources of uncertainty at once.

The uncertainty is evaluated at 1 concentration levels: 0.0098 % m/m.


#### Step 2. The data

**Precision** (random effects), 1 entry:

- **s_I(T)**: staggered_nested


#### Step 3. What the program calculates

It combines the components as u = √(u(precision)² + u(bias)² + …), at each level, and multiplies by the coverage factor (k = 2 for about 95 %) to get the expanded uncertainty U. The results show u and U, absolute and relative, at every level.


> **Try:** Compare the size of the precision and bias components in the results. If bias dominates, a better reference material or a correction helps more than repeating measurements.


#### Finally: what the source expects

Reference: ISO 5725-3:2023 Tables J.4 and J.5, level 1 (s_r = 0.381, s_I(T) = 0.603, s_R = 0.801 × 10⁻³ %).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Calibration functions with uncertainty on both axes (Advanced)

Gas analysis with straight-line, quadratic, power and exponential functions, and reference mixtures whose contents are correlated. The program tests which function fits.

<a id="iso6143-b2-n2-natural-gas-covariances"></a>

### N₂ in natural gas by GC-TCD: correlated reference mixtures (ISO 6143 Example 2)

`iso6143-b2-n2-natural-gas-covariances`

#### What is this project about?

ISO 6143:2025 B.2.3: the straight line again, now with the covariances of reference mixtures that share their history (a dilution series and a common set of primary standards).


#### Step 1. The idea: a calibration curve

An instrument gives a response (here in peak area) that is not directly the content of **N₂**. It is calibrated with 8 reference mixtures of known content. Both the contents and the responses have uncertainties, so the fit (generalised least squares, ISO 6143) uses both.

Analysis function: **linear**. The program checks whether it fits the points (the weighted residuals and Γ should be small).


#### Step 2. Unknown samples

For 2 samples, the measured response is converted into a content through the fitted function; the uncertainty includes both the response of the sample and the uncertainty of the curve.


#### Finally: what the source expects

Reference: ISO 6143:2025 B.2.3, parameters with covariances and Table B.10 (B_LEAST).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="iso6143-b2-n2-natural-gas-linear"></a>

### N₂ in natural gas by GC-TCD: straight line (ISO 6143 Example 2)

`iso6143-b2-n2-natural-gas-linear`

#### What is this project about?

ISO 6143:2025 B.2.3: generalized least-squares straight line through eight calibration points with uncertainties in both variables, the comparison with the other analysis functions and the contents of two mixtures.


#### Step 1. The idea: a calibration curve

An instrument gives a response (here in peak area) that is not directly the content of **N₂**. It is calibrated with 8 reference mixtures of known content. Both the contents and the responses have uncertainties, so the fit (generalised least squares, ISO 6143) uses both.

Analysis function: **linear**. The program checks whether it fits the points (the weighted residuals and Γ should be small).


#### Step 2. Unknown samples

For 2 samples, the measured response is converted into a content through the fitted function; the uncertainty includes both the response of the sample and the uncertainty of the curve.


#### Finally: what the source expects

Reference: ISO 6143:2025 B.2.3, linear parameters and Table B.8 (B_LEAST).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="iso6143-b2-n2-natural-gas-quadratic"></a>

### N₂ in natural gas by GC-TCD: second-order polynomial (ISO 6143 Example 2)

`iso6143-b2-n2-natural-gas-quadratic`

#### What is this project about?

ISO 6143:2025 B.2.3: the same calibration data with a second-order polynomial analysis function.


#### Step 1. The idea: a calibration curve

An instrument gives a response (here in peak area) that is not directly the content of **N₂**. It is calibrated with 8 reference mixtures of known content. Both the contents and the responses have uncertainties, so the fit (generalised least squares, ISO 6143) uses both.

Analysis function: **quadratic**. The program checks whether it fits the points (the weighted residuals and Γ should be small).


#### Step 2. Unknown samples

For 2 samples, the measured response is converted into a content through the fitted function; the uncertainty includes both the response of the sample and the uncertainty of the curve.


#### Finally: what the source expects

Reference: ISO 6143:2025 B.2.3, polynomial coefficients and Table B.9 (B_LEAST).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="iso6143-b3-no-ndir-exponential"></a>

### NO in N₂ by NDIR: exponential function (ISO 6143 Example 3)

`iso6143-b3-no-ndir-exponential`

#### What is this project about?

ISO 6143:2025 B.2.4: the same NDIR calibration with the exponential analysis function x = b₀ + b₁e^(b₂y), the function of choice in the standard; the parameter uncertainties printed there are known to be wrong.


#### Step 1. The idea: a calibration curve

An instrument gives a response (here in NDIR signal) that is not directly the content of **NO**. It is calibrated with 12 reference mixtures of known content. Both the contents and the responses have uncertainties, so the fit (generalised least squares, ISO 6143) uses both.

Analysis function: **exponential**. The program checks whether it fits the points (the weighted residuals and Γ should be small).


#### Step 2. Unknown samples

For 1 sample, the measured response is converted into a content through the fitted function; the uncertainty includes both the response of the sample and the uncertainty of the curve.


#### Finally: what the source expects

Reference: ISO 6143:2025 B.2.4, exponential parameters, S_res, Γ and x of Table B.12 (B_LEAST); u(b) and u(x) from ODRPACK (scipy.odr, cov_beta) for the same data.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="iso6143-b3-no-ndir-power"></a>

### NO in N₂ by NDIR: power function (ISO 6143 Example 3)

`iso6143-b3-no-ndir-power`

#### What is this project about?

ISO 6143:2025 B.2.4: a non-linear NDIR calibration with twelve points; the power analysis function x = b₀ + b₁y^(1+b₂) fitted by generalized least squares.


#### Step 1. The idea: a calibration curve

An instrument gives a response (here in NDIR signal) that is not directly the content of **NO**. It is calibrated with 12 reference mixtures of known content. Both the contents and the responses have uncertainties, so the fit (generalised least squares, ISO 6143) uses both.

Analysis function: **power**. The program checks whether it fits the points (the weighted residuals and Γ should be small).


#### Step 2. Unknown samples

For 1 sample, the measured response is converted into a content through the fitted function; the uncertainty includes both the response of the sample and the uncertainty of the curve.


#### Finally: what the source expects

Reference: ISO 6143:2025 B.2.4, power-function parameters and Table B.12 (B_LEAST).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## An analyser calibrated by dynamic dilution (Advanced)

The reference mixtures are made from the same parent gas, so their contents are correlated. Ignoring the covariances changes the result.

<a id="emue-e3.3-no2-analyser-dynamic-dilution-covariances-ignored"></a>

### NO₂ analyser calibrated with mixtures from dynamic dilution: analysis function with correlated reference values, covariances ignored (EMUE E3.3)

`emue-e3.3-no2-analyser-dynamic-dilution-covariances-ignored`

#### What is this project about?

Three NO₂/air mixtures (708, 974, 1290 nmol/mol) from one parent mixture diluted with two calibrated MFCs; straight-line analysis function x = A + B·w fitted with uncertainties in both variables; validation with an independent mixture of 975.5 nmol/mol.


#### Step 1. The idea: a calibration curve

An instrument gives a response (here in a.u.) that is not directly the content of **NO₂ in synthetic air**. It is calibrated with 3 reference mixtures of known content. Both the contents and the responses have uncertainties, so the fit (generalised least squares, ISO 6143) uses both.

Analysis function: **linear**. The program checks whether it fits the points (the weighted residuals and Γ should be small).


#### Step 2. Unknown samples

For 1 sample, the measured response is converted into a content through the fitted function; the uncertainty includes both the response of the sample and the uncertainty of the curve.


#### Finally: what the source expects

Reference: EMUE E3.3.6: A = 88.94 nmol/mol, B = 4.19 nmol/(mol·a.u.), V_AB (E3.3.18); validation y = 982.0 nmol/mol, u = 4.7 (4.5 without covariances). Expected values recomputed (F90).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="emue-e3.3-no2-analyser-dynamic-dilution"></a>

### NO₂ analyser calibrated with mixtures from dynamic dilution: analysis function with correlated reference values (EMUE E3.3)

`emue-e3.3-no2-analyser-dynamic-dilution`

#### What is this project about?

Three NO₂/air mixtures (708, 974, 1290 nmol/mol) from one parent mixture diluted with two calibrated MFCs; straight-line analysis function x = A + B·w fitted with uncertainties in both variables; validation with an independent mixture of 975.5 nmol/mol.


#### Step 1. The idea: a calibration curve

An instrument gives a response (here in a.u.) that is not directly the content of **NO₂ in synthetic air**. It is calibrated with 3 reference mixtures of known content. Both the contents and the responses have uncertainties, so the fit (generalised least squares, ISO 6143) uses both.

Analysis function: **linear**. The program checks whether it fits the points (the weighted residuals and Γ should be small).


#### Step 2. Unknown samples

For 1 sample, the measured response is converted into a content through the fitted function; the uncertainty includes both the response of the sample and the uncertainty of the curve.


#### Finally: what the source expects

Reference: EMUE E3.3.6: A = 88.94 nmol/mol, B = 4.19 nmol/(mol·a.u.), V_AB (E3.3.18); validation y = 982.0 nmol/mol, u = 4.7 (4.5 without covariances). Expected values recomputed (F90).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## An error approximation function (Advanced)

A straight line fitted through the errors of a balance at many loads, with a χ² test and a residual test deciding how large an extra term s_m must be.

<a id="cg18-h4-400g-error-function-sm-0.05"></a>

### Balance 400 g / 0.1 mg: error approximation function with s_m = 0.05 mg, χ² test passed (EURAMET cg-18 H4)

`cg18-h4-400g-error-function-sm-0.05`

#### What is this project about?

Error approximation function E = a₁I of a 400 g balance from nine test points with correlated reference masses.


#### Step 1. The idea: calibrating a balance

The Electronic balance, Max 400 g, d = 0.1 mg is tested with standard weights at 9 loads (EURAMET cg-18). At each load the error E = indication − reference mass and its uncertainty are found (repeatability, rounding of the display, eccentricity, the weights, air buoyancy).

A straight line (the error approximation function) is fitted through the errors, so that the error and its uncertainty are known for every load, not only the tested ones.


#### Finally: what the source expects

Reference: EURAMET cg-18 v4.0 H4.4, s_m = 0.05 mg: a₁ = 0.000 84 mg/g, U(â) = 5.637·10⁻⁸ (mg/g)², χ²_obs = 7.3; u(E_appr) and the residual test of Table H4.6.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="cg18-h4-400g-error-function-sm-0.25"></a>

### Balance 400 g / 0.1 mg: error approximation function with s_m = 0.25 mg, residual test passed (EURAMET cg-18 H4)

`cg18-h4-400g-error-function-sm-0.25`

#### What is this project about?

Error approximation function E = a₁I of a 400 g balance from nine test points with correlated reference masses.


#### Step 1. The idea: calibrating a balance

The Electronic balance, Max 400 g, d = 0.1 mg is tested with standard weights at 9 loads (EURAMET cg-18). At each load the error E = indication − reference mass and its uncertainty are found (repeatability, rounding of the display, eccentricity, the weights, air buoyancy).

A straight line (the error approximation function) is fitted through the errors, so that the error and its uncertainty are known for every load, not only the tested ones.


#### Finally: what the source expects

Reference: EURAMET cg-18 v4.0 H4.4, s_m = 0.25 mg: a₁ = 0.000 84 mg/g, U(â) = 1.745·10⁻⁷ (mg/g)², u(a₁) = 0.000 42 mg/g; u(E_appr) of Table H4.8.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="cg18-h4-400g-error-function"></a>

### Balance 400 g / 0.1 mg: error approximation function E = a₁I, first fit (EURAMET cg-18 H4)

`cg18-h4-400g-error-function`

#### What is this project about?

Error approximation function E = a₁I of a 400 g balance from nine test points with correlated reference masses.


#### Step 1. The idea: calibrating a balance

The Electronic balance, Max 400 g, d = 0.1 mg is tested with standard weights at 9 loads (EURAMET cg-18). At each load the error E = indication − reference mass and its uncertainty are found (repeatability, rounding of the display, eccentricity, the weights, air buoyancy).

A straight line (the error approximation function) is fitted through the errors, so that the error and its uncertainty are known for every load, not only the tested ones.


#### Finally: what the source expects

Reference: EURAMET cg-18 v4.0 H4.4, s_m = 0: a₁ = 0.000 83 mg/g, U(â) = 5.109·10⁻⁸ (mg/g)², u(a₁) = 0.000 23 mg/g, χ²_obs = 12.5 (test fails).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Matrix effects in a top-down model (Advanced)

Seven elements in marine sediments: recoveries from reference materials of different matrices, and a model with two concentration intervals.

<a id="palma2019-cr-marine-sediment"></a>

### Chromium in marine sediments, matrix effects in a two-interval top-down model (Palma et al. 2019)

`palma2019-cr-marine-sediment`

#### What is this project about?

Palma, Morgado and Bettencourt da Silva (2019), Table 9: uncertainty of total Cr in marine sediments from duplicate analyses, the intermediate precision of a homogeneous sample, the variability of recoveries in 36 proficiency tests (matrix effects) and the weighted mean recovery.


#### Step 1. The idea: uncertainty from the laboratory's own data

This is a **top-down** evaluation of **w(Cr)** (mg/kg). Instead of modelling every step of the method, it uses data that the laboratory already has: how much the results scatter when the method is repeated on different days (precision) and how far they are, on average, from the true value (bias or recovery). Together these cover most sources of uncertainty at once.

The uncertainty is evaluated at 2 concentration levels: 5; 50 mg/kg.


#### Step 2. The data

**Precision** (random effects), 4 entries:

- **s_r(h)⟨I⟩**: a standard deviation of 0.285 (e.g. from a control chart: within-laboratory reproducibility).
- **s_I⟨I⟩**: a standard deviation of 0.565 (e.g. from a control chart: within-laboratory reproducibility).
- **s′_r(h)⟨II⟩**: a standard deviation of 2.85 % (e.g. from a control chart: within-laboratory reproducibility).
- **s′_I⟨II⟩**: a standard deviation of 5.65 % (e.g. from a control chart: within-laboratory reproducibility).

**Further components**, added as they are given:

- u_ME⟨I⟩
- u′_ME⟨II⟩
- u_T⟨I⟩
- u′_T⟨II⟩


#### Step 3. What the program calculates

It combines the components as u = √(u(precision)² + u(bias)² + …), at each level, and multiplies by the coverage factor (k = 2 for about 95 %) to get the expanded uncertainty U. The results show u and U, absolute and relative, at every level.


> **Try:** Compare the size of the precision and bias components in the results. If bias dominates, a better reference material or a correction helps more than repeating measurements.


#### Finally: what the source expects

Reference: Palma et al. (2019) Table 9: u_c⟨I⟩ = 1.05 mg/kg, u′_c⟨II⟩ = 10.5 %; tolerance one unit of the last printed digit, since the components are themselves rounded.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="palma2019-cu-marine-sediment"></a>

### Copper in marine sediments, matrix effects in a two-interval top-down model (Palma et al. 2019)

`palma2019-cu-marine-sediment`

#### What is this project about?

Palma, Morgado and Bettencourt da Silva (2019), Table 9: uncertainty of total Cu in marine sediments from duplicate analyses, the intermediate precision of a homogeneous sample, the variability of recoveries in 36 proficiency tests (matrix effects) and the weighted mean recovery.


#### Step 1. The idea: uncertainty from the laboratory's own data

This is a **top-down** evaluation of **w(Cu)** (mg/kg). Instead of modelling every step of the method, it uses data that the laboratory already has: how much the results scatter when the method is repeated on different days (precision) and how far they are, on average, from the true value (bias or recovery). Together these cover most sources of uncertainty at once.

The uncertainty is evaluated at 2 concentration levels: 5; 50 mg/kg.


#### Step 2. The data

**Precision** (random effects), 4 entries:

- **s_r(h)⟨I⟩**: a standard deviation of 0.242 (e.g. from a control chart: within-laboratory reproducibility).
- **s_I⟨I⟩**: a standard deviation of 0.248 (e.g. from a control chart: within-laboratory reproducibility).
- **s′_r(h)⟨II⟩**: a standard deviation of 2.43 % (e.g. from a control chart: within-laboratory reproducibility).
- **s′_I⟨II⟩**: a standard deviation of 2.48 % (e.g. from a control chart: within-laboratory reproducibility).

**Further components**, added as they are given:

- u_ME⟨I⟩
- u′_ME⟨II⟩
- u_T⟨I⟩
- u′_T⟨II⟩


#### Step 3. What the program calculates

It combines the components as u = √(u(precision)² + u(bias)² + …), at each level, and multiplies by the coverage factor (k = 2 for about 95 %) to get the expanded uncertainty U. The results show u and U, absolute and relative, at every level.


> **Try:** Compare the size of the precision and bias components in the results. If bias dominates, a better reference material or a correction helps more than repeating measurements.


#### Finally: what the source expects

Reference: Palma et al. (2019) Table 9: u_c⟨I⟩ = 0.612 mg/kg, u′_c⟨II⟩ = 6.13 %; tolerance one unit of the last printed digit, since the components are themselves rounded.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="palma2019-li-marine-sediment"></a>

### Lithium in marine sediments, matrix effects in a two-interval top-down model (Palma et al. 2019)

`palma2019-li-marine-sediment`

#### What is this project about?

Palma, Morgado and Bettencourt da Silva (2019), Table 9: uncertainty of total Li in marine sediments from duplicate analyses, the intermediate precision of a homogeneous sample, the variability of recoveries in 36 proficiency tests (matrix effects) and the weighted mean recovery.


#### Step 1. The idea: uncertainty from the laboratory's own data

This is a **top-down** evaluation of **w(Li)** (mg/kg). Instead of modelling every step of the method, it uses data that the laboratory already has: how much the results scatter when the method is repeated on different days (precision) and how far they are, on average, from the true value (bias or recovery). Together these cover most sources of uncertainty at once.

The uncertainty is evaluated at 2 concentration levels: 0.5; 5 mg/kg.


#### Step 2. The data

**Precision** (random effects), 4 entries:

- **s_r(h)⟨I⟩**: a standard deviation of 0.0345 (e.g. from a control chart: within-laboratory reproducibility).
- **s_I⟨I⟩**: a standard deviation of 0.058 (e.g. from a control chart: within-laboratory reproducibility).
- **s′_r(h)⟨II⟩**: a standard deviation of 3.45 % (e.g. from a control chart: within-laboratory reproducibility).
- **s′_I⟨II⟩**: a standard deviation of 5.8 % (e.g. from a control chart: within-laboratory reproducibility).

**Further components**, added as they are given:

- u_ME⟨I⟩
- u′_ME⟨II⟩
- u_T⟨I⟩
- u′_T⟨II⟩


#### Step 3. What the program calculates

It combines the components as u = √(u(precision)² + u(bias)² + …), at each level, and multiplies by the coverage factor (k = 2 for about 95 %) to get the expanded uncertainty U. The results show u and U, absolute and relative, at every level.


> **Try:** Compare the size of the precision and bias components in the results. If bias dominates, a better reference material or a correction helps more than repeating measurements.


#### Finally: what the source expects

Reference: Palma et al. (2019) Table 9: u_c⟨I⟩ = 0.127 mg/kg, u′_c⟨II⟩ = 12.7 %; tolerance one unit of the last printed digit, since the components are themselves rounded.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="palma2019-mn-marine-sediment"></a>

### Manganese in marine sediments, matrix effects in a two-interval top-down model (Palma et al. 2019)

`palma2019-mn-marine-sediment`

#### What is this project about?

Palma, Morgado and Bettencourt da Silva (2019), Table 9: uncertainty of total Mn in marine sediments from duplicate analyses, the intermediate precision of a homogeneous sample, the variability of recoveries in 36 proficiency tests (matrix effects) and the weighted mean recovery.


#### Step 1. The idea: uncertainty from the laboratory's own data

This is a **top-down** evaluation of **w(Mn)** (mg/kg). Instead of modelling every step of the method, it uses data that the laboratory already has: how much the results scatter when the method is repeated on different days (precision) and how far they are, on average, from the true value (bias or recovery). Together these cover most sources of uncertainty at once.

The uncertainty is evaluated at 2 concentration levels: 5; 50 mg/kg.


#### Step 2. The data

**Precision** (random effects), 4 entries:

- **s_r(h)⟨I⟩**: a standard deviation of 0.179 (e.g. from a control chart: within-laboratory reproducibility).
- **s_I⟨I⟩**: a standard deviation of 0.656 (e.g. from a control chart: within-laboratory reproducibility).
- **s′_r(h)⟨II⟩**: a standard deviation of 1.79 % (e.g. from a control chart: within-laboratory reproducibility).
- **s′_I⟨II⟩**: a standard deviation of 6.56 % (e.g. from a control chart: within-laboratory reproducibility).

**Further components**, added as they are given:

- u_ME⟨I⟩
- u′_ME⟨II⟩
- u_T⟨I⟩
- u′_T⟨II⟩


#### Step 3. What the program calculates

It combines the components as u = √(u(precision)² + u(bias)² + …), at each level, and multiplies by the coverage factor (k = 2 for about 95 %) to get the expanded uncertainty U. The results show u and U, absolute and relative, at every level.


> **Try:** Compare the size of the precision and bias components in the results. If bias dominates, a better reference material or a correction helps more than repeating measurements.


#### Finally: what the source expects

Reference: Palma et al. (2019) Table 9: u_c⟨I⟩ = 0.92 mg/kg, u′_c⟨II⟩ = 9.2 %; tolerance one unit of the last printed digit, since the components are themselves rounded.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="palma2019-ni-marine-sediment"></a>

### Nickel in marine sediments, matrix effects in a two-interval top-down model (Palma et al. 2019)

`palma2019-ni-marine-sediment`

#### What is this project about?

Palma, Morgado and Bettencourt da Silva (2019), Table 9: uncertainty of acid-extractable Ni in marine sediments from duplicate analyses, the intermediate precision of a homogeneous sample, the variability of recoveries in 36 proficiency tests (matrix effects) and the weighted mean recovery.


#### Step 1. The idea: uncertainty from the laboratory's own data

This is a **top-down** evaluation of **w(Ni)** (mg/kg). Instead of modelling every step of the method, it uses data that the laboratory already has: how much the results scatter when the method is repeated on different days (precision) and how far they are, on average, from the true value (bias or recovery). Together these cover most sources of uncertainty at once.

The uncertainty is evaluated at 2 concentration levels: 7.5; 75 mg/kg.


#### Step 2. The data

**Precision** (random effects), 4 entries:

- **s_r(h)⟨I⟩**: a standard deviation of 0.346 (e.g. from a control chart: within-laboratory reproducibility).
- **s_I⟨I⟩**: a standard deviation of 1.13 (e.g. from a control chart: within-laboratory reproducibility).
- **s′_r(h)⟨II⟩**: a standard deviation of 2.58 % (e.g. from a control chart: within-laboratory reproducibility).
- **s′_I⟨II⟩**: a standard deviation of 7.54 % (e.g. from a control chart: within-laboratory reproducibility).

**Further components**, added as they are given:

- u_ME⟨I⟩
- u′_ME⟨II⟩
- u_T⟨I⟩
- u′_T⟨II⟩


#### Step 3. What the program calculates

It combines the components as u = √(u(precision)² + u(bias)² + …), at each level, and multiplies by the coverage factor (k = 2 for about 95 %) to get the expanded uncertainty U. The results show u and U, absolute and relative, at every level.


> **Try:** Compare the size of the precision and bias components in the results. If bias dominates, a better reference material or a correction helps more than repeating measurements.


#### Finally: what the source expects

Reference: Palma et al. (2019) Table 9: u_c⟨I⟩ = 1.25 mg/kg, u′_c⟨II⟩ = 8.44 %; tolerance one unit of the last printed digit, since the components are themselves rounded.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="palma2019-pb-marine-sediment"></a>

### Lead in marine sediments, matrix effects in a two-interval top-down model (Palma et al. 2019)

`palma2019-pb-marine-sediment`

#### What is this project about?

Palma, Morgado and Bettencourt da Silva (2019), Table 9: uncertainty of acid-extractable Pb in marine sediments from duplicate analyses, the intermediate precision of a homogeneous sample, the variability of recoveries in 36 proficiency tests (matrix effects) and the weighted mean recovery.


#### Step 1. The idea: uncertainty from the laboratory's own data

This is a **top-down** evaluation of **w(Pb)** (mg/kg). Instead of modelling every step of the method, it uses data that the laboratory already has: how much the results scatter when the method is repeated on different days (precision) and how far they are, on average, from the true value (bias or recovery). Together these cover most sources of uncertainty at once.

The uncertainty is evaluated at 2 concentration levels: 10; 100 mg/kg.


#### Step 2. The data

**Precision** (random effects), 4 entries:

- **s_r(h)⟨I⟩**: a standard deviation of 0.714 (e.g. from a control chart: within-laboratory reproducibility).
- **s_I⟨I⟩**: a standard deviation of 0.804 (e.g. from a control chart: within-laboratory reproducibility).
- **s′_r(h)⟨II⟩**: a standard deviation of 4.95 % (e.g. from a control chart: within-laboratory reproducibility).
- **s′_I⟨II⟩**: a standard deviation of 4.02 % (e.g. from a control chart: within-laboratory reproducibility).

**Further components**, added as they are given:

- u_ME⟨I⟩
- u′_ME⟨II⟩
- u_T⟨I⟩
- u′_T⟨II⟩


#### Step 3. What the program calculates

It combines the components as u = √(u(precision)² + u(bias)² + …), at each level, and multiplies by the coverage factor (k = 2 for about 95 %) to get the expanded uncertainty U. The results show u and U, absolute and relative, at every level.


> **Try:** Compare the size of the precision and bias components in the results. If bias dominates, a better reference material or a correction helps more than repeating measurements.


#### Finally: what the source expects

Reference: Palma et al. (2019) Table 9: u_c⟨I⟩ = 1.9 mg/kg, u′_c⟨II⟩ = 10.1 %; tolerance one unit of the last printed digit, since the components are themselves rounded.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="palma2019-zn-marine-sediment"></a>

### Zinc in marine sediments, matrix effects in a two-interval top-down model (Palma et al. 2019)

`palma2019-zn-marine-sediment`

#### What is this project about?

Palma, Morgado and Bettencourt da Silva (2019), Table 9: uncertainty of total Zn in marine sediments from duplicate analyses, the intermediate precision of a homogeneous sample, the variability of recoveries in 36 proficiency tests (matrix effects) and the weighted mean recovery.


#### Step 1. The idea: uncertainty from the laboratory's own data

This is a **top-down** evaluation of **w(Zn)** (mg/kg). Instead of modelling every step of the method, it uses data that the laboratory already has: how much the results scatter when the method is repeated on different days (precision) and how far they are, on average, from the true value (bias or recovery). Together these cover most sources of uncertainty at once.

The uncertainty is evaluated at 2 concentration levels: 2; 20 mg/kg.


#### Step 2. The data

**Precision** (random effects), 4 entries:

- **s_r(h)⟨I⟩**: a standard deviation of 0.108 (e.g. from a control chart: within-laboratory reproducibility).
- **s_I⟨I⟩**: a standard deviation of 0.117 (e.g. from a control chart: within-laboratory reproducibility).
- **s′_r(h)⟨II⟩**: a standard deviation of 2.69 % (e.g. from a control chart: within-laboratory reproducibility).
- **s′_I⟨II⟩**: a standard deviation of 2.92 % (e.g. from a control chart: within-laboratory reproducibility).

**Further components**, added as they are given:

- u_ME⟨I⟩
- u′_ME⟨II⟩
- u_T⟨I⟩
- u′_T⟨II⟩


#### Step 3. What the program calculates

It combines the components as u = √(u(precision)² + u(bias)² + …), at each level, and multiplies by the coverage factor (k = 2 for about 95 %) to get the expanded uncertainty U. The results show u and U, absolute and relative, at every level.


> **Try:** Compare the size of the precision and bias components in the results. If bias dominates, a better reference material or a correction helps more than repeating measurements.


#### Finally: what the source expects

Reference: Palma et al. (2019) Table 9: u_c⟨I⟩ = 0.365 mg/kg, u′_c⟨II⟩ = 9.13 %; tolerance one unit of the last printed digit, since the components are themselves rounded.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Arithmetic or weighted mean recovery (Advanced)

Fluoranthene in river sediments: the mean recovery calculated in two ways, as in the MUPR guide and its spreadsheet.

<a id="bettencourt2026-fluoranthene-arithmetic"></a>

### Fluoranthene in river sediments, arithmetic mean recovery (MUPR Eq. 8–12) (Bettencourt da Silva 2026)

`bettencourt2026-fluoranthene-arithmetic`

#### What is this project about?

Top-down uncertainty of fluoranthene in river sediments from the in-house validation data of the MUPR spreadsheet: a two-interval precision model and 20 mean recoveries that are not compatible, so the dispersion of the recoveries is added and results are corrected for recovery.


#### Step 1. The idea: uncertainty from the laboratory's own data

This is a **top-down** evaluation of **w(fluoranthene)** (mg/kg). Instead of modelling every step of the method, it uses data that the laboratory already has: how much the results scatter when the method is repeated on different days (precision) and how far they are, on average, from the true value (bias or recovery). Together these cover most sources of uncertainty at once.

The uncertainty is evaluated at 2 concentration levels: 0.1; 10 mg/kg.


#### Step 2. The data

**Precision** (random effects), 2 entries:

- **s_I⟨I⟩**: a standard deviation of 0.075185 (e.g. from a control chart: within-laboratory reproducibility).
- **s′_I⟨II⟩**: a standard deviation of 5.0123 % (e.g. from a control chart: within-laboratory reproducibility).

**Recovery**: 20 recovery tests (reference materials or spiked samples). The mean recovery R and its uncertainty describe how much of the analyte the method finds; if R differs significantly from 100 %, a correction is considered.


#### Step 3. What the program calculates

It combines the components as u = √(u(precision)² + u(bias)² + …), at each level, and multiplies by the coverage factor (k = 2 for about 95 %) to get the expanded uncertainty U. The results show u and U, absolute and relative, at every level.


> **Try:** Compare the size of the precision and bias components in the results. If bias dominates, a better reference material or a correction helps more than repeating measurements.


#### Finally: what the source expects

Reference: Recomputed from Supplementary Material 06 (independent script). The χ² = 40.9729, s(R) = 8.7552 % and R̿ = 74.8871 % agree with the sheet Trueness (CM16, CO16, CK16); the uncertainties differ from the spreadsheet as explained in the notes.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="bettencourt2026-fluoranthene-weighted"></a>

### Fluoranthene in river sediments, weighted mean recovery (MUPR Eq. 16.1) (Bettencourt da Silva 2026)

`bettencourt2026-fluoranthene-weighted`

#### What is this project about?

Top-down uncertainty of fluoranthene in river sediments from the in-house validation data of the MUPR spreadsheet: a two-interval precision model and 20 mean recoveries that are not compatible, so the dispersion of the recoveries is added and results are corrected for recovery.


#### Step 1. The idea: uncertainty from the laboratory's own data

This is a **top-down** evaluation of **w(fluoranthene)** (mg/kg). Instead of modelling every step of the method, it uses data that the laboratory already has: how much the results scatter when the method is repeated on different days (precision) and how far they are, on average, from the true value (bias or recovery). Together these cover most sources of uncertainty at once.

The uncertainty is evaluated at 2 concentration levels: 0.1; 10 mg/kg.


#### Step 2. The data

**Precision** (random effects), 2 entries:

- **s_I⟨I⟩**: a standard deviation of 0.075185 (e.g. from a control chart: within-laboratory reproducibility).
- **s′_I⟨II⟩**: a standard deviation of 5.0123 % (e.g. from a control chart: within-laboratory reproducibility).

**Recovery**: 20 recovery tests (reference materials or spiked samples). The mean recovery R and its uncertainty describe how much of the analyte the method finds; if R differs significantly from 100 %, a correction is considered.


#### Step 3. What the program calculates

It combines the components as u = √(u(precision)² + u(bias)² + …), at each level, and multiplies by the coverage factor (k = 2 for about 95 %) to get the expanded uncertainty U. The results show u and U, absolute and relative, at every level.


> **Try:** Compare the size of the precision and bias components in the results. If bias dominates, a better reference material or a correction helps more than repeating measurements.


#### Finally: what the source expects

Reference: Recomputed from Supplementary Material 06 (independent script). The χ² = 40.9729, s(R) = 8.7552 % and R̿ = 74.8871 % agree with the sheet Trueness (CM16, CO16, CK16); the uncertainties differ from the spreadsheet as explained in the notes.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Purity by quantitative NMR (BIPM) (Advanced)

The reference data of the BIPM for internal standards in qNMR. Complete purity budgets; one result lies close to the physical limit of 1 and is reported with an asymmetric interval.

<a id="bipm-isrd-01-qnmr-dmso2-ma-d2o"></a>

### Purity of dimethyl sulfone by ¹H qNMR with maleic acid as internal standard in D2O (BIPM ISRD-01, Table 4)

`bipm-isrd-01-qnmr-dmso2-ma-d2o`

#### What is this project about?

Mass fraction of the analyte A by quantitative NMR with an internal standard S (measurement equation 1 of the ISRD): w_A = (I_A/I_S)(N_S/N_A)(M_A/M_S)(m_S/m_A) w_S. The integral ratio is a Type A evaluation (standard deviation of the mean); the other components are Type B.


#### Step 1. What do we measure, and how is it calculated?

The result is **w_A** in g/g (Mass fraction (purity) of dimethyl sulfone). It is not measured directly: it is calculated from the input quantities with the measurement model

```
w_A = R * (N_S/N_A) * (M_A/M_S) * (m_S/m_A) * w_S
```


#### Step 2. The input quantities and their uncertainties

The model has 8 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **R**: Integral ratio I_A/I_S (mean of replicates). Best value 5.4919 1 with standard uncertainty u = 0.00096 1 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **N_A**: Number of ¹H nuclei of the quantified signal of dimethyl sulfone. Best value 5.9934 1 with standard uncertainty u = 0.0003 1 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **N_S**: Number of ¹H nuclei of the quantified signal of maleic acid. Best value 1.9994 1 with standard uncertainty u = 0.0002 1 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **M_A**: Molar mass of dimethyl sulfone. Best value 94.136 g/mol with standard uncertainty u = 0.005 g/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
5. **M_S**: Molar mass of maleic acid. Best value 116.0724 g/mol with standard uncertainty u = 0.0025 g/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
6. **m_A**: Mass of dimethyl sulfone weighed. Best value 3.5063 mg with standard uncertainty u = 0.00124 mg (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
7. **m_S**: Mass of maleic acid weighed. Best value 2.3545 mg with standard uncertainty u = 0.00124 mg (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
8. **w_S**: Purity of the internal standard maleic acid. Best value 0.9995 g/g with standard uncertainty u = 0.0003 g/g (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 0.9973 g/g
- standard uncertainty u = 0.00073 g/g

Reference: BIPM ISRD-01, Table 4: assigned value 0.9973 g/g, combined standard uncertainty 7.300e-04 g/g (k = 2 for the expanded uncertainty). Tolerance of u: 1 % or the difference to the value recomputed from the printed inputs, 0.3 % (F97).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="bipm-isrd-01-qnmr-dmtp-ma-dmso-d6"></a>

### Purity of dimethyl terephthalate by ¹H qNMR with maleic acid as internal standard in DMSO-d6 (BIPM ISRD-01, Table 6)

`bipm-isrd-01-qnmr-dmtp-ma-dmso-d6`

#### What is this project about?

Mass fraction of the analyte A by quantitative NMR with an internal standard S (measurement equation 1 of the ISRD): w_A = (I_A/I_S)(N_S/N_A)(M_A/M_S)(m_S/m_A) w_S. The integral ratio is a Type A evaluation (standard deviation of the mean); the other components are Type B.


#### Step 1. What do we measure, and how is it calculated?

The result is **w_A** in g/g (Mass fraction (purity) of dimethyl terephthalate). It is not measured directly: it is calculated from the input quantities with the measurement model

```
w_A = R * (N_S/N_A) * (M_A/M_S) * (m_S/m_A) * w_S
```


#### Step 2. The input quantities and their uncertainties

The model has 8 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **R**: Integral ratio I_A/I_S (mean of replicates). Best value 0.7501 1 with standard uncertainty u = 0.00012 1 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **N_A**: Number of ¹H nuclei of the quantified signal of dimethyl terephthalate. Best value 3.9968 1 with standard uncertainty u = 0.0002 1 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **N_S**: Number of ¹H nuclei of the quantified signal of maleic acid. Best value 1.9994 1 with standard uncertainty u = 0.0002 1 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **M_A**: Molar mass of dimethyl terephthalate. Best value 194.184 g/mol with standard uncertainty u = 0.006 g/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
5. **M_S**: Molar mass of maleic acid. Best value 116.072 g/mol with standard uncertainty u = 0.0025 g/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
6. **m_A**: Mass of dimethyl terephthalate weighed. Best value 20.3108 mg with standard uncertainty u = 0.00337 mg (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
7. **m_S**: Mass of maleic acid weighed. Best value 32.3521 mg with standard uncertainty u = 0.00337 mg (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
8. **w_S**: Purity of the internal standard maleic acid. Best value 0.9995 g/g with standard uncertainty u = 0.0003 g/g (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 0.9995 g/g
- standard uncertainty u = 0.000396 g/g

Reference: BIPM ISRD-01, Table 6: assigned value 0.9995 g/g, combined standard uncertainty 3.960e-04 g/g (k = 2 for the expanded uncertainty). Tolerance of u: 1 % or the difference to the value recomputed from the printed inputs, 3.4 % (F97).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="bipm-isrd-02-qnmr-dmtp-dmso2-cdcl3"></a>

### Purity of dimethyl terephthalate by ¹H qNMR with dimethyl sulfone as internal standard in CDCl3 (BIPM ISRD-02, Table 6)

`bipm-isrd-02-qnmr-dmtp-dmso2-cdcl3`

#### What is this project about?

Mass fraction of the analyte A by quantitative NMR with an internal standard S (measurement equation 1 of the ISRD): w_A = (I_A/I_S)(N_S/N_A)(M_A/M_S)(m_S/m_A) w_S. The integral ratio is a Type A evaluation (standard deviation of the mean); the other components are Type B.


#### Step 1. What do we measure, and how is it calculated?

The result is **w_A** in g/g (Mass fraction (purity) of dimethyl terephthalate). It is not measured directly: it is calculated from the input quantities with the measurement model

```
w_A = R * (N_S/N_A) * (M_A/M_S) * (m_S/m_A) * w_S
```


#### Step 2. The input quantities and their uncertainties

The model has 8 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **R**: Integral ratio I_A/I_S (mean of replicates). Best value 0.7154 1 with standard uncertainty u = 0.00009 1 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **N_A**: Number of ¹H nuclei of the quantified signal of dimethyl terephthalate. Best value 5.9988 1 with standard uncertainty u = 0.0003 1 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **N_S**: Number of ¹H nuclei of the quantified signal of dimethyl sulfone. Best value 5.9988 1 with standard uncertainty u = 0.0003 1 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **M_A**: Molar mass of dimethyl terephthalate. Best value 194.194 g/mol with standard uncertainty u = 0.008 g/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
5. **M_S**: Molar mass of dimethyl sulfone. Best value 94.1284 g/mol with standard uncertainty u = 0.0062 g/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
6. **m_A**: Mass of dimethyl terephthalate weighed. Best value 8.5689 mg with standard uncertainty u = 0.00124 mg (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
7. **m_S**: Mass of dimethyl sulfone weighed. Best value 5.8213 mg with standard uncertainty u = 0.00124 mg (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
8. **w_S**: Purity of the internal standard dimethyl sulfone. Best value 0.9964 g/g with standard uncertainty u = 0.0004 g/g (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 0.999 g/g
- standard uncertainty u = 0.000505 g/g

Reference: BIPM ISRD-02, Table 6: assigned value 0.999 g/g, combined standard uncertainty 5.050e-04 g/g (k = 2 for the expanded uncertainty). Tolerance of u: 1 % or the difference to the value recomputed from the printed inputs, 0.2 % (F97).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="bipm-isrd-02-qnmr-ma-dmso2-d2o"></a>

### Purity of maleic acid by ¹H qNMR with dimethyl sulfone as internal standard in D2O (BIPM ISRD-02, Table 4)

`bipm-isrd-02-qnmr-ma-dmso2-d2o`

#### What is this project about?

Mass fraction of the analyte A by quantitative NMR with an internal standard S (measurement equation 1 of the ISRD): w_A = (I_A/I_S)(N_S/N_A)(M_A/M_S)(m_S/m_A) w_S. The integral ratio is a Type A evaluation (standard deviation of the mean); the other components are Type B.


#### Step 1. What do we measure, and how is it calculated?

The result is **w_A** in g/g (Mass fraction (purity) of maleic acid). It is not measured directly: it is calculated from the input quantities with the measurement model

```
w_A = R * (N_S/N_A) * (M_A/M_S) * (m_S/m_A) * w_S
```


#### Step 2. The input quantities and their uncertainties

The model has 8 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **R**: Integral ratio I_A/I_S (mean of replicates). Best value 0.1821 1 with standard uncertainty u = 0.00003 1 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **N_A**: Number of ¹H nuclei of the quantified signal of maleic acid. Best value 1.9996 1 with standard uncertainty u = 0.0003 1 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **N_S**: Number of ¹H nuclei of the quantified signal of dimethyl sulfone. Best value 5.9988 1 with standard uncertainty u = 0.0003 1 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **M_A**: Molar mass of maleic acid. Best value 116.08 g/mol with standard uncertainty u = 0.004 g/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
5. **M_S**: Molar mass of dimethyl sulfone. Best value 94.1284 g/mol with standard uncertainty u = 0.0062 g/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
6. **m_A**: Mass of maleic acid weighed. Best value 2.3545 mg with standard uncertainty u = 0.00124 mg (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
7. **m_S**: Mass of dimethyl sulfone weighed. Best value 3.5063 mg with standard uncertainty u = 0.00124 mg (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
8. **w_S**: Purity of the internal standard dimethyl sulfone. Best value 0.9965 g/g with standard uncertainty u = 0.0004 g/g (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 0.9997 g/g
- standard uncertainty u = 0.000791 g/g

Reference: BIPM ISRD-02, Table 4: assigned value 0.9997 g/g, combined standard uncertainty 7.910e-04 g/g (k = 2 for the expanded uncertainty). Tolerance of u: 1 % or the difference to the value recomputed from the printed inputs, 0.4 % (F97).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="bipm-isrd-06-qnmr-ba-btmsb-d4-cd3od"></a>

### Purity of benzoic acid by ¹H qNMR with BTMSB-d4 as internal standard in CD3OD (BIPM ISRD-06, Table 6)

`bipm-isrd-06-qnmr-ba-btmsb-d4-cd3od`

#### What is this project about?

Mass fraction of the analyte A by quantitative NMR with an internal standard S (measurement equation 1 of the ISRD): w_A = (I_A/I_S)(N_S/N_A)(M_A/M_S)(m_S/m_A) w_S. The integral ratio is a Type A evaluation (standard deviation of the mean); the other components are Type B.


#### Step 1. What do we measure, and how is it calculated?

The result is **w_A** in g/g (Mass fraction (purity) of benzoic acid). It is not measured directly: it is calculated from the input quantities with the measurement model

```
w_A = R * (N_S/N_A) * (M_A/M_S) * (m_S/m_A) * w_S
```

The result cannot be above 1 (a physical bound, e.g. a purity cannot exceed 1). The program respects this bound when it states the interval.


#### Step 2. The input quantities and their uncertainties

The model has 8 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **R**: Integral ratio I_A/I_S (mean of replicates). Best value 1.5 1 with standard uncertainty u = 0.00026 1 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **N_A**: Number of ¹H nuclei of the quantified signal of benzoic acid. Best value 4.999 1 with standard uncertainty u = 0.0003 1 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **N_S**: Number of ¹H nuclei of the quantified signal of BTMSB-d4. Best value 17.9964 1 with standard uncertainty u = 0.0003 1 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **M_A**: Molar mass of benzoic acid. Best value 122.123 g/mol with standard uncertainty u = 0.0054 g/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
5. **M_S**: Molar mass of BTMSB-d4. Best value 226.502 g/mol with standard uncertainty u = 0.0125 g/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
6. **m_A**: Mass of benzoic acid weighed. Best value 6.4659 mg with standard uncertainty u = 0.00124 mg (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
7. **m_S**: Mass of BTMSB-d4 weighed. Best value 2.2223 mg with standard uncertainty u = 0.00124 mg (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
8. **w_S**: Purity of the internal standard BTMSB-d4. Best value 0.9991 g/g with standard uncertainty u = 0.0004 g/g (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 0.99977 g/g
- standard uncertainty u = 0.000744 g/g

Reference: BIPM ISRD-06, Table 6: assigned value 0.9998 g/g; the integral ratio is printed as 1.5 (two significant digits), which allows ±0.033 g/g, so the estimate is checked against the value recomputed from the printed inputs (independent Python, 0.999774014 g/g) instead; combined standard uncertainty 7.440e-04 g/g (k = 2 for the expanded uncertainty). Tolerance of u: 1 % or the difference to the value recomputed from the printed inputs, 0.6 % (F97).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="bipm-isrd-06-qnmr-dmso2-btmsb-d4-dmso-d6"></a>

### Purity of dimethyl sulfone by ¹H qNMR with BTMSB-d4 as internal standard in DMSO-d6 (BIPM ISRD-06, Table 4)

`bipm-isrd-06-qnmr-dmso2-btmsb-d4-dmso-d6`

#### What is this project about?

Mass fraction of the analyte A by quantitative NMR with an internal standard S (measurement equation 1 of the ISRD): w_A = (I_A/I_S)(N_S/N_A)(M_A/M_S)(m_S/m_A) w_S. The integral ratio is a Type A evaluation (standard deviation of the mean); the other components are Type B.


#### Step 1. What do we measure, and how is it calculated?

The result is **w_A** in g/g (Mass fraction (purity) of dimethyl sulfone). It is not measured directly: it is calculated from the input quantities with the measurement model

```
w_A = R * (N_S/N_A) * (M_A/M_S) * (m_S/m_A) * w_S
```


#### Step 2. The input quantities and their uncertainties

The model has 8 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **R**: Integral ratio I_A/I_S (mean of replicates). Best value 0.8173 1 with standard uncertainty u = 0.00011 1 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **N_A**: Number of ¹H nuclei of the quantified signal of dimethyl sulfone. Best value 5.9988 1 with standard uncertainty u = 0.0003 1 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **N_S**: Number of ¹H nuclei of the quantified signal of BTMSB-d4. Best value 17.9964 1 with standard uncertainty u = 0.0003 1 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **M_A**: Molar mass of dimethyl sulfone. Best value 94.128 g/mol with standard uncertainty u = 0.007 g/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
5. **M_S**: Molar mass of BTMSB-d4. Best value 226.5024 g/mol with standard uncertainty u = 0.0125 g/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
6. **m_A**: Mass of dimethyl sulfone weighed. Best value 2.6524 mg with standard uncertainty u = 0.00124 mg (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
7. **m_S**: Mass of BTMSB-d4 weighed. Best value 2.5969 mg with standard uncertainty u = 0.00124 mg (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
8. **w_S**: Purity of the internal standard BTMSB-d4. Best value 0.9991 g/g with standard uncertainty u = 0.0004 g/g (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 0.9967 g/g
- standard uncertainty u = 0.000797 g/g

Reference: BIPM ISRD-06, Table 4: assigned value 0.9967 g/g, combined standard uncertainty 7.970e-04 g/g (k = 2 for the expanded uncertainty). Tolerance of u: 1 % or the difference to the value recomputed from the printed inputs, 0.2 % (F97).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="bipm-isrd-07-qnmr-dmso2-dss-d6-d2o"></a>

### Purity of dimethyl sulfone by ¹H qNMR with DSS-d6 as internal standard in D2O (BIPM ISRD-07, Table 4)

`bipm-isrd-07-qnmr-dmso2-dss-d6-d2o`

#### What is this project about?

Mass fraction of the analyte A by quantitative NMR with an internal standard S (measurement equation 1 of the ISRD): w_A = (I_A/I_S)(N_S/N_A)(M_A/M_S)(m_S/m_A) w_S. The integral ratio is a Type A evaluation (standard deviation of the mean); the other components are Type B.


#### Step 1. What do we measure, and how is it calculated?

The result is **w_A** in g/g (Mass fraction (purity) of dimethyl sulfone). It is not measured directly: it is calculated from the input quantities with the measurement model

```
w_A = R * (N_S/N_A) * (M_A/M_S) * (m_S/m_A) * w_S
```


#### Step 2. The input quantities and their uncertainties

The model has 8 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **R**: Integral ratio I_A/I_S (mean of replicates). Best value 4.5173 1 with standard uncertainty u = 0.00057 1 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **N_A**: Number of ¹H nuclei of the quantified signal of dimethyl sulfone. Best value 5.9988 1 with standard uncertainty u = 0.0003 1 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **N_S**: Number of ¹H nuclei of the quantified signal of DSS-d6. Best value 8.9982 1 with standard uncertainty u = 0.0003 1 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **M_A**: Molar mass of dimethyl sulfone. Best value 94.128 g/mol with standard uncertainty u = 0.0069 g/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
5. **M_S**: Molar mass of DSS-d6. Best value 224.3544 g/mol with standard uncertainty u = 0.0081 g/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
6. **m_A**: Mass of dimethyl sulfone weighed. Best value 3.252 mg with standard uncertainty u = 0.00124 mg (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
7. **m_S**: Mass of DSS-d6 weighed. Best value 1.2351 mg with standard uncertainty u = 0.00124 mg (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
8. **w_S**: Purity of the internal standard DSS-d6. Best value 0.9227 g/g with standard uncertainty u = 0.00045 g/g (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 0.9962 g/g
- standard uncertainty u = 0.00119 g/g

Reference: BIPM ISRD-07, Table 4: assigned value 0.9962 g/g, combined standard uncertainty 1.190e-03 g/g (k = 2 for the expanded uncertainty). Tolerance of u: 1 % or the difference to the value recomputed from the printed inputs, 0.3 % (F97).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="bipm-isrd-07-qnmr-ma-dss-d6-d2o"></a>

### Purity of maleic acid by ¹H qNMR with DSS-d6 as internal standard in D2O (BIPM ISRD-07, Table 6)

`bipm-isrd-07-qnmr-ma-dss-d6-d2o`

#### What is this project about?

Mass fraction of the analyte A by quantitative NMR with an internal standard S (measurement equation 1 of the ISRD): w_A = (I_A/I_S)(N_S/N_A)(M_A/M_S)(m_S/m_A) w_S. The integral ratio is a Type A evaluation (standard deviation of the mean); the other components are Type B.


#### Step 1. What do we measure, and how is it calculated?

The result is **w_A** in g/g (Mass fraction (purity) of maleic acid). It is not measured directly: it is calculated from the input quantities with the measurement model

```
w_A = R * (N_S/N_A) * (M_A/M_S) * (m_S/m_A) * w_S
```


#### Step 2. The input quantities and their uncertainties

The model has 8 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **R**: Integral ratio I_A/I_S (mean of replicates). Best value 1.7512 1 with standard uncertainty u = 0.00031 1 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **N_A**: Number of ¹H nuclei of the quantified signal of maleic acid. Best value 1.9996 1 with standard uncertainty u = 0.0003 1 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **N_S**: Number of ¹H nuclei of the quantified signal of DSS-d6. Best value 8.9982 1 with standard uncertainty u = 0.0003 1 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **M_A**: Molar mass of maleic acid. Best value 116.072 g/mol with standard uncertainty u = 0.004 g/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
5. **M_S**: Molar mass of DSS-d6. Best value 224.354 g/mol with standard uncertainty u = 0.0081 g/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
6. **m_A**: Mass of maleic acid weighed. Best value 5.1383 mg with standard uncertainty u = 0.00124 mg (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
7. **m_S**: Mass of DSS-d6 weighed. Best value 1.3641 mg with standard uncertainty u = 0.00124 mg (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
8. **w_S**: Purity of the internal standard DSS-d6. Best value 0.9227 g/g with standard uncertainty u = 0.00045 g/g (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 0.9987 g/g
- standard uncertainty u = 0.00108 g/g

Reference: BIPM ISRD-07, Table 6: assigned value 0.9987 g/g, combined standard uncertainty 1.080e-03 g/g (k = 2 for the expanded uncertainty). Tolerance of u: 1 % or the difference to the value recomputed from the printed inputs, 0.4 % (F97).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="bipm-isrd-08-qnmr-btmsb-d4-ba-cd3od"></a>

### Purity of BTMSB-d4 by ¹H qNMR with benzoic acid as internal standard in CD3OD (BIPM ISRD-08, Table 2)

`bipm-isrd-08-qnmr-btmsb-d4-ba-cd3od`

#### What is this project about?

Mass fraction of the analyte A by quantitative NMR with an internal standard S (measurement equation 1 of the ISRD): w_A = (I_A/I_S)(N_S/N_A)(M_A/M_S)(m_S/m_A) w_S. The integral ratio is a Type A evaluation (standard deviation of the mean); the other components are Type B.


#### Step 1. What do we measure, and how is it calculated?

The result is **w_A** in g/g (Mass fraction (purity) of BTMSB-d4). It is not measured directly: it is calculated from the input quantities with the measurement model

```
w_A = R * (N_S/N_A) * (M_A/M_S) * (m_S/m_A) * w_S
```


#### Step 2. The input quantities and their uncertainties

The model has 8 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **R**: Integral ratio I_A/I_S (mean of replicates). Best value 1.4313 1 with standard uncertainty u = 0.00032 1 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **N_A**: Number of ¹H nuclei of the quantified signal of BTMSB-d4. Best value 17.9964 1 with standard uncertainty u = 0.0003 1 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **N_S**: Number of ¹H nuclei of the quantified signal of benzoic acid. Best value 1.9996 1 with standard uncertainty u = 0.0003 1 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **M_A**: Molar mass of BTMSB-d4. Best value 226.502 g/mol with standard uncertainty u = 0.013 g/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
5. **M_S**: Molar mass of benzoic acid. Best value 122.123 g/mol with standard uncertainty u = 0.006 g/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
6. **m_A**: Mass of BTMSB-d4 weighed. Best value 2.2271 mg with standard uncertainty u = 0.00029 mg (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
7. **m_S**: Mass of benzoic acid weighed. Best value 7.5422 mg with standard uncertainty u = 0.00029 mg (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
8. **w_S**: Purity of the internal standard benzoic acid. Best value 0.99992 g/g with standard uncertainty u = 0.000025 g/g (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 0.9988 g/g
- standard uncertainty u = 0.00031 g/g

Reference: BIPM ISRD-08, Table 2: assigned value 0.9988 g/g, combined standard uncertainty 3.100e-04 g/g (k = 2 for the expanded uncertainty). Tolerance of u: 1 % or the difference to the value recomputed from the printed inputs, 0.6 % (F97).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Isotope dilution (IDMS) (Advanced)

Lead in water by double isotope dilution: a long model with isotope ratios, masses and a calibration standard. A reference method with a small uncertainty.

<a id="quam-a7-assay-standard"></a>

### Lead assay standard for IDMS

`quam-a7-assay-standard`

#### What is this project about?

QUAM Example A7, Eq. (8): amount content of the primary lead assay standard prepared gravimetrically.


#### Step 1. What do we measure, and how is it calculated?

The result is **c_z** in µmol/g (Amount content of the assay standard). It is not measured directly: it is calculated from the input quantities with the measurement model

```
c_z = 1e6 * m2 / d2 * m1 * w / d1 / M
```


#### Step 2. The input quantities and their uncertainties

The model has 6 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **m1**: Mass of lead. Best value 0.36544 g with standard uncertainty u = 0.00005 g (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **d1**: First dilution (total mass). Best value 196.14 g with standard uncertainty u = 0.03 g (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **m2**: Aliquot of the first dilution. Best value 1.0292 g with standard uncertainty u = 0.0002 g (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **d2**: Second dilution (total mass). Best value 99.931 g with standard uncertainty u = 0.01 g (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
5. **w**: Purity of the lead. Best value 0.99999 with standard uncertainty u = 0.000005 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
6. **M**: Molar mass of lead in the assay material. Best value 207.2104 g/mol with standard uncertainty u = 0.001 g/mol (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 0.092605 µmol/g
- standard uncertainty u = 0.000028 µmol/g

Reference: QUAM:2012 Example A7, Table A7.3.

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

<a id="quam-a7-lead-double-idms"></a>

### Lead in water by double isotope dilution

`quam-a7-lead-double-idms`

#### What is this project about?

QUAM Example A7, Eq. (5): double IDMS with mass-bias-corrected isotope ratios (replicate 1).


#### Step 1. What do we measure, and how is it calculated?

The result is **c_x** in µmol/g (Amount content of lead in the water sample). It is not measured directly: it is calculated from the input quantities with the measurement model

```
c_x = c_z * m_y/m_x * m_z/m_y2 * (K_y1*R_y1 - K_b*R_b)/(K_b*R_b - K_x1*R_x1) * (K_b2*R_b2 - K_z1*R_z1)/(K_y1*R_y1 - K_b2*R_b2) * sum_x/sum_z - c_blank
```

To keep the equation readable, 11 intermediate quantities are calculated first:

- `K_b = K0_b + Kbias_b`
- `K_b2 = K0_b2 + Kbias_b2`
- `K_x1 = K0_x1 + Kbias_x1`
- `K_x3 = K0_x3 + Kbias_x3`
- `K_x4 = K0_x4 + Kbias_x4`
- `K_y1 = K0_y1 + Kbias_y1`
- `K_z1 = K0_z1 + Kbias_z1`
- `K_z3 = K0_z3 + Kbias_z3`
- `K_z4 = K0_z4 + Kbias_z4`
- `sum_x = K_x1*R_x1 + 1 + K_x3*R_x3 + K_x4*R_x4`
- `sum_z = K_z1*R_z1 + 1 + K_z3*R_z3 + K_z4*R_z4`


#### Step 2. The input quantities and their uncertainties

The model has 33 inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:

1. **c_z**: Amount content of the assay standard. Best value 0.092605 µmol/g with standard uncertainty u = 0.000028 µmol/g (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
2. **m_x**: Mass of sample in blend b. Best value 1.044 g with standard uncertainty u = 0.0002 g (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
3. **m_y**: Mass of spike in blend b. Best value 1.136 g with standard uncertainty u = 0.0002 g (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
4. **m_y2**: Mass of spike in blend b'. Best value 1.0654 g with standard uncertainty u = 0.0002 g (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
5. **m_z**: Mass of assay standard in blend b'. Best value 1.1029 g with standard uncertainty u = 0.0002 g (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
6. **K0_b**: Mass-bias correction factor (measured); u = 0.0025/√8. Best value 0.9987 with standard uncertainty u = 0.00088388 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
7. **K0_b2**: Mass-bias correction factor (measured); u = 0.0025/√8. Best value 0.9983 with standard uncertainty u = 0.00088388 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
8. **K0_x1**: Mass-bias correction factor (measured); u = 0.0025/√8. Best value 0.9992 with standard uncertainty u = 0.00088388 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
9. **K0_x3**: Mass-bias correction factor (measured); u = 0.0035/√8. Best value 1.0004 with standard uncertainty u = 0.0012374 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
10. **K0_x4**: Mass-bias correction factor (measured); u = 0.006/√8. Best value 1.001 with standard uncertainty u = 0.0021213 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
11. **K0_y1**: Mass-bias correction factor (measured); u = 0.0025/√8. Best value 0.9999 with standard uncertainty u = 0.00088388 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
12. **K0_z1**: Mass-bias correction factor (measured); u = 0.0025/√8. Best value 0.9989 with standard uncertainty u = 0.00088388 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
13. **K0_z3**: Mass-bias correction factor (measured); u = 0.0035/√8. Best value 0.9993 with standard uncertainty u = 0.0012374 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
14. **K0_z4**: Mass-bias correction factor (measured); u = 0.006/√8. Best value 1.0002 with standard uncertainty u = 0.0021213 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
15. **R_b**: Measured isotope amount ratio; u = 0.00073/√8. Best value 0.2936 with standard uncertainty u = 0.00025809 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
16. **R_b2**: Measured isotope amount ratio; u = 0.0013/√8. Best value 0.505 with standard uncertainty u = 0.00045962 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
17. **R_x1**: Measured isotope amount ratio; u = 0.0054/√8. Best value 2.1402 with standard uncertainty u = 0.0019092 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
18. **R_x3**: Measured isotope amount ratio; u = 0.0032/√8. Best value 0.9142 with standard uncertainty u = 0.0011314 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
19. **R_x4**: Measured isotope amount ratio; u = 0.00035/√8. Best value 0.05901 with standard uncertainty u = 0.00012374 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
20. **R_y1**: Measured isotope amount ratio; u = 4e-05/√8. Best value 0.00064 with standard uncertainty u = 1.4142 × 10⁻⁵ (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
21. **R_z1**: Measured isotope amount ratio; u = 0.0054/√8. Best value 2.1429 with standard uncertainty u = 0.0019092 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
22. **R_z3**: Measured isotope amount ratio; u = 0.0032/√8. Best value 0.9147 with standard uncertainty u = 0.0011314 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
23. **R_z4**: Measured isotope amount ratio; u = 0.00035/√8. Best value 0.0587 with standard uncertainty u = 0.00012374 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
24. **c_blank**: Procedure blank; u = 4.0 × 10⁻⁷/√8. Best value 4.5e-7 µmol/g with standard uncertainty u = 1.4142 × 10⁻⁷ µmol/g (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
25. **Kbias_b**: Bias of the mass-bias correction. Best value 0 with standard uncertainty u = 0.001 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
26. **Kbias_b2**: Bias of the mass-bias correction. Best value 0 with standard uncertainty u = 0.001 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
27. **Kbias_x1**: Bias of the mass-bias correction. Best value 0 with standard uncertainty u = 0.001 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
28. **Kbias_x3**: Bias of the mass-bias correction. Best value 0 with standard uncertainty u = 0.001 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
29. **Kbias_x4**: Bias of the mass-bias correction. Best value 0 with standard uncertainty u = 0.001 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
30. **Kbias_y1**: Bias of the mass-bias correction. Best value 0 with standard uncertainty u = 0.001 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
31. **Kbias_z1**: Bias of the mass-bias correction. Best value 0 with standard uncertainty u = 0.001 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
32. **Kbias_z3**: Bias of the mass-bias correction. Best value 0 with standard uncertainty u = 0.001 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).
33. **Kbias_z4**: Bias of the mass-bias correction. Best value 0 with standard uncertainty u = 0.001 (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).


#### Step 3. What the program calculates

1. It puts the estimates of the inputs into the equation: this gives the estimate of the result.
2. For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).
3. It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.
4. It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.
5. It multiplies u_c by the coverage factor k = 2: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).

It also uses the **Monte Carlo method**: it draws 1 000 000 random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the 1 000 000 results. If the two methods agree, the simpler GUM result can be trusted.


> **Try:** Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.


#### Finally: what the source expects

With the GUM method the source gives:

- estimate y = 0.053738 µmol/g
- standard uncertainty u = 0.00018 µmol/g

Reference: QUAM:2012 Example A7, Table A7.8 (replicate 1).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.

## Gy's sampling theory (Advanced)

The fundamental sampling error of an enzyme in chicken feed, estimated from particle size and properties.

<a id="ufs-a5-enzyme-feed-gy"></a>

### Enzyme in chicken feed: fundamental sampling error by Gy's theory (Eurachem UfS Example A5)

`ufs-a5-enzyme-feed-gy`

#### What is this project about?

A 500 g primary sample from a 25 kg bag (0.05 % m/m enzyme), ground to < 0.5 mm, and a 2 g test portion for extraction and LC (5 % analytical RSD). Modelling approach with the material properties of Table A5.1.


#### Step 1. The idea: the uncertainty starts at sampling

A result describes a whole lot, field or batch, but only a small sample is analysed. The **duplicate method** (Eurachem UfS) takes two independent samples from several targets and analyses each twice. An analysis of variance then separates the variation due to **sampling** from that due to **analysis**.

Here, the fundamental sampling error is also estimated from the properties of the particles (Gy's theory).


> **Try:** Compare u(sampling) with u(analysis) in the results: often the sampling part is the larger one, and a better analysis would not help.


#### Finally: what the source expects

Reference: Values of Section 5 and the summary of Example A5 (c, C₁, C₂, s_r1, s_r2, s_t; U′ 26.8 % sampling, 28.6 % total).

When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.
