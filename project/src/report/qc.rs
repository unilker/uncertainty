//! Reports of control charts for internal quality control (Nordtest TR 569).

use super::*;
use crate::qc::{BasisSpec, CenterSource, ControlChartAnalysis, SdSource};
use uncertainty_core::qc::{ChartKind, Status};

pub(super) struct QcLabels {
    pub title: &'static str,
    pub sample: &'static str,
    pub chart: &'static str,
    pub kinds: [&'static str; 3],
    pub replicates: &'static str,
    pub center: &'static str,
    pub center_computed: &'static str,
    pub center_given: &'static str,
    pub center_given_range: &'static str,
    pub center_reference: &'static str,
    pub center_expected: &'static str,
    pub sd: &'static str,
    pub sd_computed: &'static str,
    pub sd_mean_range: &'static str,
    pub sd_pooled: &'static str,
    pub sd_given: &'static str,
    pub sd_target: &'static str,
    pub basis: [&'static str; 3],
    pub limits: &'static str,
    pub lower: &'static str,
    pub upper: &'static str,
    pub warning: &'static str,
    pub action: &'static str,
    pub action_only: &'static str,
    pub excluded: &'static str,
    pub establish: &'static str,
    pub daily: &'static str,
    pub daily_rules: &'static str,
    pub daily_rules_action: &'static str,
    pub daily_summary: &'static str,
    pub run: &'static str,
    pub label: &'static str,
    pub value: &'static str,
    pub status: &'static str,
    pub statuses: [&'static str; 4],
    pub all_in_control: &'static str,
    pub statistical: &'static str,
    pub statistical_text: &'static str,
    pub trend: &'static str,
    pub run_rule: &'static str,
    pub none: &'static str,
    pub bias: &'static str,
    pub bias_text: &'static str,
    pub significant: &'static str,
    pub not_significant: &'static str,
    pub review: &'static str,
    pub review_text: &'static str,
    pub previous: &'static str,
    pub current: &'static str,
    pub computed: &'static str,
    pub given: &'static str,
    pub mean: &'static str,
    pub n: &'static str,
    pub dof: &'static str,
    pub f_test: &'static str,
    pub t_test: &'static str,
    pub critical: &'static str,
    pub shift: &'static str,
    pub shift_text: &'static str,
    pub outside_wl: &'static str,
    pub outside_wl_text: &'static str,
    pub beyond: &'static str,
    pub sd_changed: &'static str,
    pub sd_unchanged: &'static str,
    pub mean_changed: &'static str,
    pub mean_unchanged: &'static str,
    pub precision: &'static str,
    pub precision_text: &'static str,
    pub anova: &'static str,
    pub eq10: &'static str,
    pub eq9: &'static str,
    pub simplified: &'static str,
    pub chart_caption: &'static str,
    pub limits_caption: &'static str,
    pub tolerance: &'static str,
    pub mean_of: &'static str,
    pub statement_limits: &'static str,
    pub statement_values: &'static str,
}

const QC_EN: QcLabels = QcLabels {
    title: "Control chart (Nordtest TR 569)",
    sample: "Control sample",
    chart: "Chart",
    kinds: ["X-chart", "R-chart (range)", "r%-chart (relative range)"],
    replicates: "replicates per range",
    center: "Central line",
    center_computed: "mean of the control values of the establishing period",
    center_given: "mean value established earlier",
    center_given_range: "mean range established earlier",
    center_reference: "reference or nominal value",
    center_expected: "expected range d₂·s",
    sd: "Standard deviation for the limits",
    sd_computed: "standard deviation of the control values (Equation 2)",
    sd_mean_range: "mean range / d₂ (Equation 4, Table 4)",
    sd_pooled: "pooled within-run standard deviation (Equation 9)",
    sd_given: "established earlier",
    sd_target: "target from the requirement",
    basis: ["statistical control limits", "target control limits", "action limits at the tolerance"],
    limits: "Control limits",
    lower: "Lower",
    upper: "Upper",
    warning: "Warning limits",
    action: "Action limits",
    action_only: "A chart with action limits only: less sensitive to a drift of the mean, since the rule two out of three is not available (Chapter 5).",
    excluded: "Runs left out of the statistics",
    establish: "The first {n} control values establish the central line and the statistical limits; the later values are judged against them.",
    daily: "Daily interpretation (Chapter 9)",
    daily_rules: "In control: within the warning limits, or between warning and action limits with neither of the two previous values outside the warning limit on the same side. Out of control: outside the action limits, or two out of three outside the warning limit on the same side.",
    daily_rules_action: "In control: within the action limits. Out of control: outside the action limits.",
    daily_summary: "{n} control values: {ok} within the warning limits, {w} between warning and action limits (in control), {oc} out of control.",
    run: "Run",
    label: "Date",
    value: "Control value",
    status: "Interpretation",
    statuses: ["in control", "in control (warning zone)", "out of control: outside the action limit", "out of control: two out of three"],
    all_in_control: "All control values are within the warning limits.",
    statistical: "Out of statistical control (Chapter 9)",
    statistical_text: "Seven consecutive values steadily increasing or decreasing, or ten out of eleven on the same side of the central line. The results can still be reported; look for the cause in the long-term review.",
    trend: "trend of seven",
    run_rule: "ten of eleven on one side",
    none: "No signals.",
    bias: "Mean against the reference value",
    bias_text: "Equation 6",
    significant: "significant",
    not_significant: "not significant",
    review: "Long-term review (Chapter 10)",
    review_text: "The control values of the period reviewed are compared with those the limits were based on: the standard deviations by an F-test (Equation 5, critical value F₁₋α/₂ of Table 3), the means by a t-test with the pooled standard deviation (Equations 7 and 9).",
    previous: "Limits based on",
    current: "Period reviewed",
    computed: "computed",
    given: "given",
    mean: "Mean",
    n: "n",
    dof: "ν",
    f_test: "F = s₁²/s₂²",
    t_test: "t (Equation 7)",
    critical: "critical",
    shift: "Shift of the mean",
    shift_text: "with about 60 values, a shift below 0.37 s needs no action",
    outside_wl: "Values outside the warning limits",
    outside_wl_text: "with about 60 values, more than 6 indicate that the standard deviation has probably increased",
    beyond: "excluded as more than {k} s from the central line",
    sd_changed: "The standard deviation has changed significantly: consider new control limits (Chapter 10).",
    sd_unchanged: "No significant change of the standard deviation.",
    mean_changed: "The mean has changed significantly: change the central line only if the shift is explained (Chapter 10).",
    mean_unchanged: "No significant change of the mean.",
    precision: "Precision from the replicate control results",
    precision_text: "Each run is analysed in replicate. The standard deviation of all results together underestimates s_Rw (Example 12).",
    anova: "One-way analysis of variance",
    eq10: "s_r pooled over the runs (Equation 10)",
    eq9: "s_r pooled over the runs (Equation 9)",
    simplified: "simplified s_Rw: standard deviations of the 1st, 2nd, … results over the runs, pooled (Example 12)",
    chart_caption: "Control chart: control values (●, out of control ■), central line, warning zone and action zone",
    limits_caption: "Central line and control limits of the chart",
    tolerance: "tolerance",
    mean_of: "mean of {k} results",
    statement_limits: "{chart}: central line {cl}; {limits} ({basis})",
    statement_values: "{n} control values: {oc} out of control, {w} between warning and action limits",
};

const QC_TR: QcLabels = QcLabels {
    title: "Kontrol grafiği (Nordtest TR 569)",
    sample: "Kontrol numunesi",
    chart: "Grafik",
    kinds: ["X-grafiği", "R-grafiği (açıklık)", "r%-grafiği (göreli açıklık)"],
    replicates: "açıklık başına tekrar",
    center: "Merkez çizgisi",
    center_computed: "kuruluş dönemindeki kontrol değerlerinin ortalaması",
    center_given: "daha önce belirlenmiş ortalama değer",
    center_given_range: "daha önce belirlenmiş ortalama açıklık",
    center_reference: "referans veya nominal değer",
    center_expected: "beklenen açıklık d₂·s",
    sd: "Sınırlar için standart sapma",
    sd_computed: "kontrol değerlerinin standart sapması (Eşitlik 2)",
    sd_mean_range: "ortalama açıklık / d₂ (Eşitlik 4, Tablo 4)",
    sd_pooled: "birleştirilmiş seri içi standart sapma (Eşitlik 9)",
    sd_given: "daha önce belirlenmiş",
    sd_target: "gereklilikten hedef değer",
    basis: ["istatistiksel kontrol sınırları", "hedef kontrol sınırları", "toleransta eylem sınırları"],
    limits: "Kontrol sınırları",
    lower: "Alt",
    upper: "Üst",
    warning: "Uyarı sınırları",
    action: "Eylem sınırları",
    action_only: "Yalnızca eylem sınırları olan grafik: üçte iki kuralı uygulanamadığından ortalamadaki kaymaya daha az duyarlıdır (Bölüm 5).",
    excluded: "İstatistiklere katılmayan seriler",
    establish: "İlk {n} kontrol değeri merkez çizgisini ve istatistiksel sınırları belirler; sonraki değerler bunlara göre değerlendirilir.",
    daily: "Günlük değerlendirme (Bölüm 9)",
    daily_rules: "Kontrol altında: uyarı sınırları içinde veya uyarı ile eylem sınırları arasında ve önceki iki değerin hiçbiri aynı tarafta uyarı sınırının dışında değil. Kontrol dışı: eylem sınırlarının dışında veya üç değerden ikisi aynı tarafta uyarı sınırının dışında.",
    daily_rules_action: "Kontrol altında: eylem sınırları içinde. Kontrol dışı: eylem sınırlarının dışında.",
    daily_summary: "{n} kontrol değeri: {ok} değer uyarı sınırları içinde, {w} değer uyarı ile eylem sınırları arasında (kontrol altında), {oc} değer kontrol dışı.",
    run: "Seri",
    label: "Tarih",
    value: "Kontrol değeri",
    status: "Değerlendirme",
    statuses: ["kontrol altında", "kontrol altında (uyarı bölgesi)", "kontrol dışı: eylem sınırı dışında", "kontrol dışı: üçte iki"],
    all_in_control: "Tüm kontrol değerleri uyarı sınırları içindedir.",
    statistical: "İstatistiksel kontrol dışı (Bölüm 9)",
    statistical_text: "Art arda yedi değerin sürekli artması veya azalması ya da on bir değerden onunun merkez çizgisinin aynı tarafında olması. Sonuçlar yine de raporlanabilir; nedeni uzun dönem değerlendirmesinde araştırın.",
    trend: "yedili eğilim",
    run_rule: "on birden onu aynı tarafta",
    none: "Sinyal yok.",
    bias: "Ortalamanın referans değerle karşılaştırılması",
    bias_text: "Eşitlik 6",
    significant: "anlamlı",
    not_significant: "anlamlı değil",
    review: "Uzun dönem değerlendirmesi (Bölüm 10)",
    review_text: "Değerlendirilen dönemin kontrol değerleri, sınırların dayandığı değerlerle karşılaştırılır: standart sapmalar F-testiyle (Eşitlik 5, Tablo 3'ün F₁₋α/₂ kritik değeri), ortalamalar birleştirilmiş standart sapmayla t-testiyle (Eşitlik 7 ve 9).",
    previous: "Sınırların dayandığı",
    current: "Değerlendirilen dönem",
    computed: "hesaplanan",
    given: "verilen",
    mean: "Ortalama",
    n: "n",
    dof: "ν",
    f_test: "F = s₁²/s₂²",
    t_test: "t (Eşitlik 7)",
    critical: "kritik",
    shift: "Ortalamadaki kayma",
    shift_text: "yaklaşık 60 değerle 0,37 s'den küçük bir kayma eylem gerektirmez",
    outside_wl: "Uyarı sınırları dışındaki değerler",
    outside_wl_text: "yaklaşık 60 değerle 6'dan fazlası standart sapmanın muhtemelen arttığını gösterir",
    beyond: "merkez çizgisinden {k} s'den uzak olduğu için dışarıda bırakıldı",
    sd_changed: "Standart sapma anlamlı olarak değişmiştir: yeni kontrol sınırları düşünülmelidir (Bölüm 10).",
    sd_unchanged: "Standart sapmada anlamlı bir değişiklik yok.",
    mean_changed: "Ortalama anlamlı olarak değişmiştir: merkez çizgisini yalnızca kayma açıklanabiliyorsa değiştirin (Bölüm 10).",
    mean_unchanged: "Ortalamada anlamlı bir değişiklik yok.",
    precision: "Tekrarlı kontrol sonuçlarından kesinlik",
    precision_text: "Her seri tekrarlı analiz edilir. Tüm sonuçların birlikte standart sapması s_Rw'yi olduğundan küçük verir (Örnek 12).",
    anova: "Tek yönlü varyans analizi",
    eq10: "serilerden birleştirilmiş s_r (Eşitlik 10)",
    eq9: "serilerden birleştirilmiş s_r (Eşitlik 9)",
    simplified: "basitleştirilmiş s_Rw: serilerdeki 1., 2., … sonuçların standart sapmaları, birleştirilmiş (Örnek 12)",
    chart_caption: "Kontrol grafiği: kontrol değerleri (●, kontrol dışı ■), merkez çizgisi, uyarı bölgesi ve eylem bölgesi",
    limits_caption: "Grafiğin merkez çizgisi ve kontrol sınırları",
    tolerance: "tolerans",
    mean_of: "{k} sonucun ortalaması",
    statement_limits: "{chart}: merkez çizgisi {cl}; {limits} ({basis})",
    statement_values: "{n} kontrol değeri: {oc} kontrol dışı, {w} uyarı ile eylem sınırları arasında",
};

pub(super) fn qc_labels(lang: Lang) -> &'static QcLabels {
    match lang {
        Lang::En => &QC_EN,
        Lang::Tr => &QC_TR,
    }
}

fn kind_name(a: &ControlChartAnalysis, d: &QcLabels) -> &'static str {
    match a.kind {
        ChartKind::X => d.kinds[0],
        ChartKind::Range => d.kinds[1],
        ChartKind::RelativeRange => d.kinds[2],
    }
}

fn unit_of(a: &ControlChartAnalysis) -> String {
    match (a.kind, &a.unit) {
        (ChartKind::RelativeRange, _) => " %".into(),
        (_, Some(u)) => format!(" {u}"),
        _ => String::new(),
    }
}

/// Significant digits for the central line and limits: enough to resolve
/// s/100, one figure more than s/10 (control values are reported with one
/// extra significant figure, Chapter 12).
fn digits(a: &ControlChartAnalysis) -> usize {
    let scale = a.limits.upper_action.abs().max(a.center.abs()).max(a.limits.lower_action.map_or(0.0, f64::abs));
    let step = a.sd.unwrap_or((a.limits.upper_action - a.center).abs() / 3.0) / 100.0;
    if !(scale > 0.0 && step > 0.0) {
        return 4;
    }
    ((scale / step).log10().ceil() as usize).clamp(3, 7)
}

/// The central line and limits, and the counts of the daily interpretation.
pub(super) fn statements(a: &ControlChartAnalysis, lang: Lang) -> Vec<String> {
    let d = qc_labels(lang);
    let unit = unit_of(a);
    let n = |x: f64| fmt_num(x, digits(a), lang);
    let mut limits = Vec::new();
    if let Some(h) = a.limits.upper_warning {
        limits.push(match a.limits.lower_warning {
            Some(l) => format!("{} {} {}{unit}", d.warning.to_lowercase(), n(l), n(h)),
            None => format!("{} {}{unit}", d.warning.to_lowercase(), n(h)),
        });
    }
    limits.push(match a.limits.lower_action {
        Some(l) => format!("{} {} {}{unit}", d.action.to_lowercase(), n(l), n(a.limits.upper_action)),
        None => format!("{} {}{unit}", d.action.to_lowercase(), n(a.limits.upper_action)),
    });
    let basis = match a.basis {
        BasisSpec::Statistical => d.basis[0],
        BasisSpec::Target => d.basis[1],
        BasisSpec::Tolerance => d.basis[2],
    };
    let mut basis = basis.to_string();
    if let Some(s) = a.sd {
        basis = format!("s = {}{unit}, {basis}", fmt_num(s, 3, lang));
    }
    let mut out = vec![d
        .statement_limits
        .replace("{chart}", kind_name(a, d))
        .replace("{cl}", &format!("{}{unit}", n(a.center)))
        .replace("{limits}", &limits.join("; "))
        .replace("{basis}", &basis)];
    if !a.values.is_empty() {
        let w = a.daily.iter().filter(|&&s| s == Status::Warning).count();
        out.push(
            d.statement_values
                .replace("{n}", &a.values.len().to_string())
                .replace("{oc}", &a.out_of_control().to_string())
                .replace("{w}", &w.to_string()),
        );
    }
    if let Some(r) = &a.review {
        let f = format!(
            "F = {} ({} {}: {})",
            fmt_num(r.review.f, 4, lang),
            d.critical,
            fmt_num(r.review.f_critical, 3, lang),
            if r.review.sd_significant() { d.significant } else { d.not_significant }
        );
        let t = if r.with_means {
            format!(
                "; t = {} ({} {}: {})",
                fmt_num(r.review.t, 4, lang),
                d.critical,
                fmt_num(r.review.t_critical, 3, lang),
                if r.review.mean_significant() { d.significant } else { d.not_significant }
            )
        } else {
            String::new()
        };
        out.push(format!("{}: {f}{t}", d.review.split(" (").next().unwrap_or(d.review)));
    }
    if let Some(p) = &a.precision {
        out.push(format!(
            "s_r = {}{unit}, s_I = {}{unit}, s_Rw = {}{unit} ({})",
            fmt_num(p.anova.s_r, 3, lang),
            fmt_num(p.anova.s_between, 3, lang),
            fmt_num(p.anova.s_rw, 3, lang),
            d.anova.to_lowercase()
        ));
    }
    out
}

fn status_text(s: Status, d: &QcLabels) -> &'static str {
    match s {
        Status::InControl => d.statuses[0],
        Status::Warning => d.statuses[1],
        Status::OutsideAction => d.statuses[2],
        Status::TwoOfThree => d.statuses[3],
    }
}

pub(super) fn markdown(
    p: &Project,
    a: &ControlChartAnalysis,
    checks: &[CheckOutcome],
    lang: Lang,
    generated: Option<&str>,
) -> String {
    let l = labels(lang);
    let d = qc_labels(lang);
    let unit = unit_of(a);
    let n = |x: f64| fmt_num(x, digits(a), lang);
    let mut s = String::new();
    let _ = writeln!(s, "# {}\n", p.metadata.title.get(lang));
    let _ = writeln!(s, "*{}*\n", l.report);
    if let Some(x) = &p.metadata.description {
        let _ = writeln!(s, "{}\n", x.get(lang));
    }
    let _ = writeln!(s, "## {}\n", d.title);
    let _ = writeln!(s, "| | |\n|---|---|");
    let unit_b = a.unit.clone().map(|u| format!(" [{u}]")).unwrap_or_default();
    let _ = writeln!(s, "| {} | {}{unit_b} |", d.sample, a.quantity);
    let chart = match (a.kind, a.replicates) {
        (ChartKind::X, Some(k)) if k > 1 => {
            format!("{} ({})", kind_name(a, d), d.mean_of.replace("{k}", &k.to_string()))
        }
        (ChartKind::X, _) => kind_name(a, d).to_string(),
        (_, Some(k)) => format!("{}, {k} {}", kind_name(a, d), d.replicates),
        (_, None) => kind_name(a, d).to_string(),
    };
    let _ = writeln!(s, "| {} | {chart} |", d.chart);
    let src = match a.center_source {
        CenterSource::Computed => d.center_computed,
        CenterSource::GivenMean if a.kind != ChartKind::X => d.center_given_range,
        CenterSource::GivenMean => d.center_given,
        CenterSource::Reference => d.center_reference,
        CenterSource::ExpectedRange => d.center_expected,
    };
    let _ = writeln!(s, "| {} | {}{unit} ({src}) |", d.center, n(a.center));
    if let Some(sd) = a.sd {
        let src = match a.sd_source {
            SdSource::Computed => d.sd_computed,
            SdSource::MeanRange => d.sd_mean_range,
            SdSource::Pooled => d.sd_pooled,
            SdSource::Given => d.sd_given,
            SdSource::Target => d.sd_target,
            SdSource::None => "",
        };
        let extra = match (a.sd_source, a.factors, a.stats) {
            (SdSource::MeanRange, Some(f), st) => {
                format!(
                    ": {} / {}",
                    fmt_num(st.map_or(a.center, |st| st.mean), digits(a), lang),
                    fmt_num(f.d2, 4, lang)
                )
            }
            (SdSource::Computed, _, Some(st)) => format!(", n = {}", st.n),
            _ => String::new(),
        };
        let _ = writeln!(s, "| {} | {}{unit} ({src}{extra}) |", d.sd, fmt_num(sd, 4, lang));
    }
    let basis = match a.basis {
        BasisSpec::Statistical => d.basis[0],
        BasisSpec::Target => d.basis[1],
        BasisSpec::Tolerance => d.basis[2],
    };
    let _ = writeln!(s, "| {} | {basis} |\n", d.limits);

    let _ = writeln!(s, "| | {} | {} | |\n|---|---:|---:|---|", d.lower, d.upper);
    let f = a.factors;
    if a.limits.has_warning_limits() {
        let rule = match f {
            None => "CL ± 2s".to_string(),
            Some(f) => format!("D_WL·s = {}·s", fmt_num(f.warning, 4, lang)),
        };
        let lo = a.limits.lower_warning.map_or("—".into(), n);
        let _ = writeln!(s, "| {} | {lo} | {} | {rule} |", d.warning, a.limits.upper_warning.map_or("—".into(), n));
    }
    let rule = match (a.basis, a.action_factor, f) {
        (BasisSpec::Tolerance, _, None) => format!("CL ± {}", d.tolerance),
        (BasisSpec::Tolerance, _, Some(_)) => d.tolerance.to_string(),
        (_, Some(k), None) => format!("CL ± {}s", fmt_num(k, 3, lang)),
        (_, Some(k), Some(_)) => format!("{}·s", fmt_num(k, 4, lang)),
        (_, None, None) => "CL ± 3s".to_string(),
        (_, None, Some(f)) => format!("D_AL·s = {}·s", fmt_num(f.action, 4, lang)),
    };
    let _ = writeln!(
        s,
        "| {} | {} | {} | {rule} |\n",
        d.action,
        a.limits.lower_action.map_or("—".into(), n),
        n(a.limits.upper_action)
    );
    if !a.limits.has_warning_limits() {
        let _ = writeln!(s, "{}\n", d.action_only);
    }
    if a.establish < a.values.len() {
        let _ = writeln!(s, "{}\n", d.establish.replace("{n}", &a.establish.to_string()));
    }
    if !a.excluded.is_empty() {
        let runs: Vec<String> = a.excluded.iter().map(|i| (i + 1).to_string()).collect();
        let _ = writeln!(s, "{}: {}.\n", d.excluded, runs.join(", "));
    }

    if !a.values.is_empty() {
        let _ = writeln!(s, "## {}\n", d.daily);
        let _ = writeln!(s, "{}\n", if a.limits.has_warning_limits() { d.daily_rules } else { d.daily_rules_action });
        let ok = a.daily.iter().filter(|&&x| x == Status::InControl).count();
        let w = a.daily.iter().filter(|&&x| x == Status::Warning).count();
        let _ = writeln!(
            s,
            "{}\n",
            d.daily_summary
                .replace("{n}", &a.values.len().to_string())
                .replace("{ok}", &ok.to_string())
                .replace("{w}", &w.to_string())
                .replace("{oc}", &a.out_of_control().to_string())
        );
        let flagged: Vec<usize> = (0..a.values.len()).filter(|&i| a.daily[i] != Status::InControl).collect();
        if flagged.is_empty() {
            let _ = writeln!(s, "{}\n", d.all_in_control);
        } else {
            let with_labels = !a.labels.is_empty();
            let head = if with_labels {
                format!("| {} | {} | {} | {} |\n|---:|---|---:|---|", d.run, d.label, d.value, d.status)
            } else {
                format!("| {} | {} | {} |\n|---:|---:|---|", d.run, d.value, d.status)
            };
            let _ = writeln!(s, "{head}");
            for i in flagged {
                let v = format!("{}{unit}", fmt_given(a.values[i], lang));
                if with_labels {
                    let _ = writeln!(s, "| {} | {} | {v} | {} |", i + 1, a.labels[i], status_text(a.daily[i], d));
                } else {
                    let _ = writeln!(s, "| {} | {v} | {} |", i + 1, status_text(a.daily[i], d));
                }
            }
            let _ = writeln!(s);
        }
        let _ = writeln!(s, "### {}\n", d.statistical);
        let _ = writeln!(s, "{}\n", d.statistical_text);
        let sig: Vec<String> = (0..a.values.len())
            .filter(|&i| a.signals[i].trend || a.signals[i].run)
            .map(|i| {
                let mut why = Vec::new();
                if a.signals[i].trend {
                    why.push(d.trend);
                }
                if a.signals[i].run {
                    why.push(d.run_rule);
                }
                format!("{} {} ({})", d.run, i + 1, why.join(", "))
            })
            .collect();
        if sig.is_empty() {
            let _ = writeln!(s, "{}\n", d.none);
        } else {
            let _ = writeln!(s, "{}\n", sig.join("; "));
        }
        if let (Some((r, t, tc)), Some(st)) = (a.bias_test, a.stats) {
            let _ = writeln!(s, "### {}\n", d.bias);
            let _ = writeln!(
                s,
                "x̄ = {}{unit}, T = {}{unit}, x̄ − T = {}{unit} ({}). t = |x̄ − T|·√n/s = {} ({}); {} {} (ν = {}): {}.\n",
                n(st.mean),
                n(r),
                fmt_num(st.mean - r, 3, lang),
                fmt_pct((st.mean - r) / r, 3, lang, true),
                fmt_num(t, 3, lang),
                d.bias_text,
                d.critical,
                fmt_num(tc, 3, lang),
                st.dof,
                if t > tc { d.significant } else { d.not_significant }
            );
        }
    }

    if let Some(r) = &a.review {
        let _ = writeln!(s, "## {}\n", d.review);
        let _ = writeln!(s, "{}\n", d.review_text);
        let src = |c: bool| if c { d.computed } else { d.given };
        let mean = |x: f64| if x.is_finite() { format!("{}{unit}", n(x)) } else { "—".into() };
        let _ = writeln!(s, "| | {} | {} | s | {} |\n|---|---:|---:|---:|---:|", d.n, d.mean, d.dof);
        for (name, x, c) in
            [(d.previous, &r.previous, r.previous_computed), (d.current, &r.current, r.current_computed)]
        {
            let _ = writeln!(
                s,
                "| {name} ({}) | {} | {} | {}{unit} | {} |",
                src(c),
                x.n,
                mean(x.mean),
                fmt_num(x.sd, 4, lang),
                fmt_given(x.dof, lang)
            );
        }
        let _ = writeln!(s);
        if !r.excluded.is_empty() {
            let runs: Vec<String> = r.excluded.iter().map(|i| (i + 1).to_string()).collect();
            let _ = writeln!(
                s,
                "{} {}: {}.\n",
                d.run,
                runs.join(", "),
                d.beyond.replace("{k}", &fmt_given(r.exclude_beyond, lang))
            );
        }
        let rv = &r.review;
        let _ = writeln!(s, "| | | {} | |\n|---|---:|---:|---|", d.critical);
        let _ = writeln!(
            s,
            "| {} | {} | {} (ν₁ = {}, ν₂ = {}) | {} |",
            d.f_test,
            fmt_num(rv.f, 4, lang),
            fmt_num(rv.f_critical, 3, lang),
            fmt_given(rv.f_dof.0, lang),
            fmt_given(rv.f_dof.1, lang),
            if rv.sd_significant() { d.significant } else { d.not_significant }
        );
        if r.with_means {
            let _ = writeln!(
                s,
                "| {} (s_p = {}{unit}) | {} | {} (ν = {}) | {} |",
                d.t_test,
                fmt_num(rv.pooled_sd, 4, lang),
                fmt_num(rv.t, 4, lang),
                fmt_num(rv.t_critical, 3, lang),
                fmt_given(rv.pooled_dof, lang),
                if rv.mean_significant() { d.significant } else { d.not_significant }
            );
            let _ = writeln!(
                s,
                "| {} | {} s | {} s | {} |",
                d.shift,
                fmt_num(rv.mean_shift, 2, lang),
                fmt_given(0.37, lang),
                d.shift_text
            );
        }
        if let (Some(ow), Some(np)) = (r.outside_warning, r.period) {
            let _ = writeln!(s, "| {} | {ow} / {np} | 6 / 60 | {} |", d.outside_wl, d.outside_wl_text);
        }
        let _ = writeln!(s);
        let _ = writeln!(s, "{}", if rv.sd_significant() { d.sd_changed } else { d.sd_unchanged });
        if r.with_means {
            let _ = writeln!(s, "{}", if rv.mean_significant() { d.mean_changed } else { d.mean_unchanged });
        }
        let _ = writeln!(s);
    }

    if let Some(pr) = &a.precision {
        let an = &pr.anova;
        let _ = writeln!(s, "## {}\n", d.precision);
        let _ = writeln!(s, "{}\n", d.precision_text);
        let _ = writeln!(s, "| | | ν |\n|---|---:|---:|");
        let _ =
            writeln!(s, "| {} — s_r | {}{unit} | {} |", d.anova, fmt_num(an.s_r, 4, lang), fmt_given(an.dof_r, lang));
        let _ = writeln!(s, "| {} — s_I | {}{unit} | {} |", d.anova, fmt_num(an.s_between, 4, lang), an.runs - 1);
        let _ = writeln!(s, "| {} — s_Rw = √(s_r² + s_I²) | {}{unit} | |", d.anova, fmt_num(an.s_rw, 4, lang));
        let _ = writeln!(s, "| {} | {}{unit} | |", d.eq10, fmt_num(pr.s_r_equal, 4, lang));
        if (pr.s_r_equal - pr.s_r_pooled).abs() > 1e-12 * pr.s_r_pooled.abs() {
            let _ = writeln!(s, "| {} | {}{unit} | |", d.eq9, fmt_num(pr.s_r_pooled, 4, lang));
        }
        if let Some(x) = pr.s_rw_simplified {
            let _ = writeln!(s, "| {} | {}{unit} | |", d.simplified, fmt_num(x, 4, lang));
        }
        let _ = writeln!(s);
    }
    tail(&mut s, p, checks, lang, "Nordtest TR 569 ed. 6 (2026)", generated);
    s
}

/// The control chart with its zones (SVG); the zones alone when no control
/// values are given.
pub(super) fn chart_svg(a: &ControlChartAnalysis, lang: Lang) -> Option<String> {
    let d = qc_labels(lang);
    let nv = a.values.len();
    let lim = &a.limits;
    let half = (lim.upper_action - a.center).abs().max(f64::MIN_POSITIVE);
    let mut lo = lim.lower_action.unwrap_or(0.0_f64.min(a.center)) - 0.25 * half;
    let mut hi = lim.upper_action + 0.25 * half;
    for &v in &a.values {
        lo = lo.min(v);
        hi = hi.max(v);
    }
    if a.kind != ChartKind::X {
        // Ranges are never negative: the axis starts at zero.
        lo = 0.0;
    }
    let pad = 0.04 * (hi - lo);
    let (lo, hi) = (if a.kind == ChartKind::X { lo - pad } else { lo }, hi + pad);
    if hi.is_nan() || lo.is_nan() || hi <= lo {
        return None;
    }
    let (w, h, left, right, top, bottom) = (640.0, 280.0, 64.0, 12.0, 10.0, 30.0);
    let plot_w = w - left - right;
    let plot_h = h - top - bottom;
    let x_of = |i: usize| left + plot_w * if nv == 1 { 0.5 } else { i as f64 / (nv - 1) as f64 };
    let y_of = |v: f64| top + plot_h * (hi - v.clamp(lo, hi)) / (hi - lo);
    let mut s = format!(
        r#"<svg viewBox="0 0 {w} {h}" width="100%" role="img" aria-label="{}" xmlns="http://www.w3.org/2000/svg">"#,
        esc(if nv == 0 { d.limits_caption } else { d.chart_caption })
    );
    let band = |s: &mut String, a: f64, b: f64, class: &str| {
        let (ya, yb) = (y_of(a.max(b)), y_of(a.min(b)));
        if yb > ya {
            let _ = write!(
                s,
                r#"<rect x="{left}" y="{ya:.1}" width="{plot_w:.1}" height="{:.1}" class="{class}"/>"#,
                yb - ya
            );
        }
    };
    // Zones: inside the warning limits, between warning and action limits,
    // beyond the action limits.
    let inner_hi = lim.upper_warning.unwrap_or(lim.upper_action);
    let inner_lo = if a.kind == ChartKind::X { lim.lower_warning.or(lim.lower_action).unwrap_or(lo) } else { lo };
    band(&mut s, inner_lo, inner_hi, "qcok");
    if let Some(uw) = lim.upper_warning {
        band(&mut s, uw, lim.upper_action, "qcwarn");
    }
    band(&mut s, lim.upper_action, hi, "qcact");
    if let (Some(lw), Some(la)) = (lim.lower_warning, lim.lower_action) {
        band(&mut s, la, lw, "qcwarn");
    }
    if let Some(la) = lim.lower_action {
        band(&mut s, lo, la, "qcact");
    }
    // Limit labels and central line.
    let mut marks: Vec<(f64, &str)> = vec![(a.center, "CL"), (lim.upper_action, "AL")];
    if let Some(v) = lim.upper_warning {
        marks.push((v, "WL"));
    }
    if let Some(v) = lim.lower_warning {
        marks.push((v, "WL"));
    }
    if let Some(v) = lim.lower_action {
        marks.push((v, "AL"));
    }
    for (v, tag) in marks {
        let y = y_of(v);
        let _ = write!(
            s,
            r#"<text x="{}" y="{:.1}" text-anchor="end" class="val">{tag} {}</text>"#,
            left - 4.0,
            y + 4.0,
            fmt_num(v, digits(a), lang)
        );
    }
    let yc = y_of(a.center);
    let _ = write!(s, r#"<line x1="{left}" x2="{:.1}" y1="{yc:.1}" y2="{yc:.1}" class="qccl"/>"#, left + plot_w);
    if a.establish < nv && a.establish > 0 {
        let x = 0.5 * (x_of(a.establish - 1) + x_of(a.establish));
        let _ = write!(s, r#"<line x1="{x:.1}" x2="{x:.1}" y1="{top}" y2="{:.1}" class="gumline"/>"#, top + plot_h);
    }
    if nv > 0 {
        let pts: Vec<String> = (0..nv).map(|i| format!("{:.1},{:.1}", x_of(i), y_of(a.values[i]))).collect();
        let _ = write!(s, r#"<polyline points="{}" class="qcline"/>"#, pts.join(" "));
    }
    let r = if nv > 120 { 1.6 } else { 3.0 };
    for i in 0..nv {
        let (x, y) = (x_of(i), y_of(a.values[i]));
        if a.daily[i].out_of_control() {
            let _ = write!(
                s,
                r#"<rect x="{:.1}" y="{:.1}" width="{:.1}" height="{:.1}" class="qcoc"/>"#,
                x - r - 0.5,
                y - r - 0.5,
                2.0 * r + 1.0,
                2.0 * r + 1.0
            );
        } else {
            let _ = write!(s, r#"<circle cx="{x:.1}" cy="{y:.1}" r="{r}" class="qcpt"/>"#);
        }
    }
    // x axis: run numbers or labels at a few positions.
    let ticks = 6.min(nv);
    let mut shown: Vec<usize> = (0..ticks).map(|k| if ticks == 1 { 0 } else { k * (nv - 1) / (ticks - 1) }).collect();
    shown.dedup();
    for i in shown {
        let text = a.labels.get(i).cloned().unwrap_or_else(|| (i + 1).to_string());
        let _ = write!(
            s,
            r#"<text x="{:.1}" y="{:.1}" text-anchor="middle" class="val">{}</text>"#,
            x_of(i),
            h - 10.0,
            esc(&text)
        );
    }
    s.push_str("</svg>");
    Some(s)
}
