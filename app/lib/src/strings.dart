// Interface texts in English and Turkish. Technical terms follow the glossary
// of the user guide (docs/guide/*/08-*.md).

class S {
  const S(this.lang);
  final String lang;

  bool get tr => lang == 'tr';

  static const Map<String, List<String>> _t = {
    // key: [English, Turkish]
    'app': ['Uncertainty Calculations', 'Belirsizlik Hesaplamaları'],
    'search': ['Search examples', 'Örneklerde ara'],
    'new_model': ['New measurement model', 'Yeni ölçüm modeli'],
    'new_model_sub': ['GUM and Monte Carlo evaluation', 'GUM ve Monte Carlo değerlendirmesi'],
    'new_topdown': ['New top-down evaluation', 'Yeni top-down değerlendirme'],
    'new_topdown_sub': ['From validation and quality-control data', 'Validasyon ve kalite kontrol verilerinden'],
    'open': ['Open project file', 'Proje dosyası aç'],
    'open_sub': ['A .json project file', 'Bir .json proje dosyası'],
    'library': ['Example library', 'Örnek kütüphanesi'],
    'examples': ['examples', 'örnek'],
    'about': ['About', 'Hakkında'],
    'cite': ['Please cite', 'Lütfen atıf yapınız'],
    'close': ['Close', 'Kapat'],
    'language': ['Language', 'Dil'],
    'overview': ['Overview', 'Genel'],
    'model': ['Model', 'Model'],
    'data': ['Data', 'Veriler'],
    'settings': ['Settings', 'Ayarlar'],
    'json': ['JSON', 'JSON'],
    'results': ['Results', 'Sonuçlar'],
    'run': ['Calculate', 'Hesapla'],
    'running': ['Calculating…', 'Hesaplanıyor…'],
    'save': ['Save', 'Kaydet'],
    'save_as': ['Save as…', 'Farklı kaydet…'],
    'export_html': ['Export report (HTML)', 'Raporu dışa aktar (HTML)'],
    'export_md': ['Export report (Markdown)', 'Raporu dışa aktar (Markdown)'],
    'saved': ['Saved', 'Kaydedildi'],
    'unsaved': ['Unsaved changes', 'Kaydedilmemiş değişiklikler'],
    'invalid': ['The project is not valid', 'Proje geçerli değil'],
    'title': ['Title', 'Başlık'],
    'title_en': ['Title (English)', 'Başlık (İngilizce)'],
    'title_tr': ['Title (Turkish)', 'Başlık (Türkçe)'],
    'description': ['Description', 'Tanım'],
    'explanation': ['Explanation', 'Açıklama'],
    'notes': ['Notes', 'Notlar'],
    'sources': ['Sources', 'Kaynaklar'],
    'output': ['Output quantity', 'Çıktı büyüklüğü'],
    'name': ['Name', 'Ad'],
    'unit': ['Unit', 'Birim'],
    'expression': ['Model equation', 'Model denklemi'],
    'intermediates': ['Intermediate quantities', 'Ara büyüklükler'],
    'inputs': ['Input quantities', 'Girdi büyüklükleri'],
    'add_input': ['Add input', 'Girdi ekle'],
    'add': ['Add', 'Ekle'],
    'remove': ['Remove', 'Kaldır'],
    'distribution': ['Distribution', 'Dağılım'],
    'dof': ['Degrees of freedom', 'Serbestlik derecesi'],
    'correlations': ['Correlations', 'Korelasyonlar'],
    'observations': ['Observations (one per line)', 'Gözlemler (her satıra bir)'],
    'gum': ['GUM uncertainty framework (JCGM 100)', 'GUM belirsizlik yaklaşımı (JCGM 100)'],
    'mcm': ['Monte Carlo method (JCGM 101)', 'Monte Carlo yöntemi (JCGM 101)'],
    'coverage': ['Coverage', 'Kapsam'],
    'by_probability': ['Coverage probability (k from t)', 'Kapsam olasılığı (k, t dağılımından)'],
    'by_factor': ['Fixed coverage factor', 'Sabit kapsam faktörü'],
    'p': ['Coverage probability', 'Kapsam olasılığı'],
    'k': ['Coverage factor', 'Kapsam faktörü'],
    'higher_order': ['Second-order terms (GUM 5.1.2 note)', 'İkinci mertebe terimler (GUM 5.1.2 notu)'],
    'mcm_on': ['Run the Monte Carlo method', 'Monte Carlo yöntemini çalıştır'],
    'trials': ['Number of trials', 'Deneme sayısı'],
    'adaptive': ['Adaptive number of trials (JCGM 101 7.9)', 'Uyarlamalı deneme sayısı (JCGM 101 7.9)'],
    'seed': ['Random-number seed', 'Rastgele sayı tohumu'],
    'interval': ['Coverage interval', 'Kapsam aralığı'],
    'shortest': ['shortest', 'en kısa'],
    'symmetric': ['probabilistically symmetric', 'olasılıksal simetrik'],
    'reporting': ['Reporting', 'Raporlama'],
    'sig_digits': ['Significant digits of U', 'U için anlamlı basamak'],
    'rounding': ['Rounding', 'Yuvarlama'],
    'half_even': ['half to even (MUPR §12)', 'yarım çifte (MUPR §12)'],
    'half_up': ['half up', 'yarım yukarı'],
    'up': ['always up', 'her zaman yukarı'],
    'apply': ['Apply', 'Uygula'],
    'json_help': [
      'The complete project file. Edit and apply; the engine checks every field.',
      'Proje dosyasının tamamı. Düzenleyip uygulayın; motor her alanı denetler.'
    ],
    'not_run': ['Press “Calculate” to evaluate the project.', 'Projeyi değerlendirmek için “Hesapla”ya basın.'],
    'reported': ['Reported result', 'Raporlanan sonuç'],
    'estimate': ['Estimate', 'Tahmin'],
    'std_unc': ['Standard uncertainty', 'Standart belirsizlik'],
    'combined': ['Combined standard uncertainty', 'Birleşik standart belirsizlik'],
    'eff_dof': ['Effective degrees of freedom', 'Etkin serbestlik derecesi'],
    'expanded': ['Expanded uncertainty', 'Genişletilmiş belirsizlik'],
    'budget': ['Uncertainty budget', 'Belirsizlik bütçesi'],
    'quantity': ['Quantity', 'Büyüklük'],
    'sensitivity': ['Sensitivity coefficient', 'Duyarlılık katsayısı'],
    'contribution': ['Contribution', 'Katkı'],
    'share': ['Share of variance', 'Varyans payı'],
    'contributions_chart': ['Contributions to the combined variance', 'Birleşik varyansa katkılar'],
    'histogram': ['Distribution of the output quantity', 'Çıktı büyüklüğünün dağılımı'],
    'mean': ['Mean', 'Ortalama'],
    'sd': ['Standard deviation', 'Standart sapma'],
    'validation': ['Validation of the GUM result (JCGM 101 clause 8)', 'GUM sonucunun doğrulanması (JCGM 101 Madde 8)'],
    'validated': ['validated', 'doğrulandı'],
    'not_validated': ['not validated', 'doğrulanmadı'],
    'tolerance': ['numerical tolerance', 'sayısal tolerans'],
    'checks': ['Comparison with reference values', 'Referans değerlerle karşılaştırma'],
    'computed': ['Computed', 'Hesaplanan'],
    'expected': ['Reference', 'Referans'],
    'passed': ['passed', 'başarılı'],
    'failed': ['FAILED', 'BAŞARISIZ'],
    'warnings': ['Warnings', 'Uyarılar'],
    'quantity_name': ['Measurand', 'Ölçülen büyüklük'],
    'levels': ['Measured values (levels)', 'Ölçülen değerler (düzeyler)'],
    'levels_help': [
      'Separated by spaces; leave empty for a relative uncertainty',
      'Boşlukla ayrılmış; bağıl belirsizlik için boş bırakın'
    ],
    'precision': ['Precision (within-laboratory reproducibility)', 'Kesinlik (laboratuvar içi tekrarüretilebilirlik)'],
    'bias': ['Bias', 'Sapma (bias)'],
    'recovery': ['Recovery', 'Geri kazanım'],
    'components': ['Further components', 'Ek bileşenler'],
    'add_precision': ['Add precision component', 'Kesinlik bileşeni ekle'],
    'data_type': ['Data', 'Veri'],
    'relative': ['relative (per cent)', 'bağıl (yüzde)'],
    'rel_kind': ['relative', 'bağıl'],
    'range': ['Range', 'Aralık'],
    'absolute': ['absolute', 'mutlak'],
    'values': ['Values (one per line)', 'Değerler (her satıra bir)'],
    'pairs': ['Duplicate pairs (one pair per line: x1 x2)', 'Mükerrer çiftler (her satıra bir çift: x1 x2)'],
    'points': ['Points (one per line: c s)', 'Noktalar (her satıra bir: c s)'],
    'range_min': ['Range from', 'Aralık başlangıcı'],
    'range_max': ['Range up to (excl.)', 'Aralık sonu (hariç)'],
    'entry_json': ['Entry (JSON)', 'Kayıt (JSON)'],
    'level': ['Measured value', 'Ölçülen değer'],
    'result': ['Result', 'Sonuç'],
    'rel_expanded': ['Relative expanded uncertainty', 'Bağıl genişletilmiş belirsizlik'],
    'rel_standard': ['Relative standard uncertainty', 'Bağıl standart belirsizlik'],
    'type': ['Type', 'Tür'],
    'significant': ['significantly different from 1: corrected', '1\'den anlamlı olarak farklı: düzeltildi'],
    'not_significant': ['not significantly different from 1', '1\'den anlamlı olarak farklı değil'],
    'mean_recovery': ['Mean recovery', 'Ortalama geri kazanım'],
    'rms_bias': ['RMS of the biases', 'Sapmaların RMS değeri'],
    'u_ref': ['Mean reference uncertainty', 'Ortalama referans belirsizliği'],
    'file_error': ['The file could not be read', 'Dosya okunamadı'],
    'form': ['Form', 'Form'],
    'kragten': ['Kragten spreadsheet view (QUAM Appendix E.2)', 'Kragten tablolama görünümü (QUAM Ek E.2)'],
    'label': ['Label', 'Etiket'],
    'bias_crm': ['certified reference materials', 'sertifikalı referans malzemeler'],
    'bias_proficiency_tests': ['proficiency tests', 'yeterlilik testleri'],
    'bias_recovery': ['recovery experiments', 'geri kazanım deneyleri'],
    'add_reference': ['Add reference', 'Referans ekle'],
    'shared_u_ref': ['shared u(C_ref) (%)', 'ortak u(C_ref) (%)'],
    'measured': ['Results (one per line)', 'Sonuçlar (her satıra bir)'],
    'values_percent': ['Recoveries in % (one per line)', 'Geri kazanımlar, % (her satıra bir)'],
    'correction': ['Correction', 'Düzeltme'],
    'corr_if_significant': ['if significant (MUPR Eq. 13)', 'anlamlıysa (MUPR Eş. 13)'],
    'corr_always': ['always', 'her zaman'],
    'corr_never': ['never', 'hiçbir zaman'],
    'include_sd': ['include s(R)', 's(R) dahil'],
    'add_test': ['Add recovery test', 'Geri kazanım deneyi ekle'],
    'confidence': ['Confidence level', 'Güven düzeyi'],
    'test_reference': ['reference material / spiked blank', 'referans malzeme / katkılı kör'],
    'test_spiked': ['spiked sample', 'katkılı numune'],
    'test_recoveries': ['list of recoveries', 'geri kazanım listesi'],
    'engine_error': ['Calculation error', 'Hesaplama hatası'],
  };

  String operator [](String key) {
    final v = _t[key];
    if (v == null) return key;
    return tr ? v[1] : v[0];
  }

  /// A bilingual text of a project file ({"en", "tr"} or a plain string).
  String text(dynamic t) {
    if (t == null) return '';
    if (t is String) return t;
    if (t is Map) return (tr ? (t['tr'] ?? t['en']) : t['en'])?.toString() ?? '';
    return t.toString();
  }

  static const fields = {
    'chemistry': ['Chemistry', 'Kimya'],
    'electrical': ['Electrical and RF', 'Elektrik ve RF'],
    'flow': ['Flow and volume', 'Akış ve hacim'],
    'general': ['General (method validation)', 'Genel (yöntem doğrulama)'],
    'length': ['Length', 'Uzunluk'],
    'mass': ['Mass', 'Kütle'],
    'temperature': ['Temperature', 'Sıcaklık'],
  };

  String field(String? f) {
    final v = fields[f ?? ''];
    if (v == null) return f ?? '';
    return tr ? v[1] : v[0];
  }

  static const distributions = {
    'normal': ['normal', 'normal'],
    't': ['t (scaled, shifted)', 't (ölçekli, kaydırılmış)'],
    'rectangular': ['rectangular', 'dikdörtgen'],
    'triangular': ['triangular', 'üçgen'],
    'trapezoidal': ['trapezoidal', 'yamuk'],
    'curvilinear_trapezoid': ['curvilinear trapezoid', 'eğrisel yamuk'],
    'arcsine': ['arc sine (U-shaped)', 'ark sinüs (U biçimli)'],
    'exponential': ['exponential', 'üstel'],
    'gamma': ['gamma', 'gama'],
    'type_a': ['Type A (observations)', 'A tipi (gözlemler)'],
    'type_a_pooled': ['Type A, pooled', 'A tipi, birleştirilmiş'],
    'constant': ['constant', 'sabit'],
  };

  String distribution(String type) {
    final v = distributions[type];
    if (v == null) return type;
    return tr ? v[1] : v[0];
  }

  static const precisionTypes = {
    'series': ['control-chart series', 'kontrol grafiği serisi'],
    'duplicates': ['duplicate pairs', 'mükerrer çiftler'],
    'groups': ['pooled groups', 'birleştirilmiş gruplar'],
    'control_limits': ['control limits', 'kontrol sınırları'],
    'given': ['stated value', 'verilen değer'],
    'linear': ['linear model s = a + b·c', 'doğrusal model s = a + b·c'],
    'continuous': ['continuous model', 'sürekli model'],
  };

  String precisionType(String type) {
    final v = precisionTypes[type];
    if (v == null) return type;
    return tr ? v[1] : v[0];
  }

  static const parameterLabels = {
    'mean': ['mean', 'ortalama'],
    'sd': ['standard uncertainty', 'standart belirsizlik'],
    'expanded': ['expanded uncertainty U', 'genişletilmiş belirsizlik U'],
    'k': ['coverage factor k', 'kapsam faktörü k'],
    'scale': ['scale', 'ölçek'],
    'dof': ['degrees of freedom', 'serbestlik derecesi'],
    'value': ['value', 'değer'],
    'half_width': ['half-width a', 'yarı genişlik a'],
    'lower': ['lower limit', 'alt sınır'],
    'upper': ['upper limit', 'üst sınır'],
    'beta': ['β (top/base)', 'β (tepe/taban)'],
    'd': ['d (limit uncertainty)', 'd (sınır belirsizliği)'],
    'shape': ['shape α', 'biçim α'],
    'pooled_sd': ['pooled standard deviation', 'birleştirilmiş standart sapma'],
    'pooled_dof': ['pooled degrees of freedom', 'birleştirilmiş serbestlik derecesi'],
    'n': ['number of observations n', 'gözlem sayısı n'],
    'sd_percent': ['standard deviation (%)', 'standart sapma (%)'],
    'limit': ['control limit L', 'kontrol sınırı L'],
    'limit_percent': ['control limit L (%)', 'kontrol sınırı L (%)'],
    'sd0': ['s₀', 's₀'],
    'rsd1_percent': ['s′₁ (%)', 's′₁ (%)'],
    'intercept': ['intercept a', 'kesim noktası a'],
    'slope': ['slope b', 'eğim b'],
    'dilution_factor': ['dilution factor F', 'seyreltme faktörü F'],
    'reference': ['reference value C', 'referans değer C'],
    'u_reference': ['u(C)', 'u(C)'],
    'u_reference_percent': ['u(C) (%)', 'u(C) (%)'],
    'sd_reproducibility_percent': ['s_R (%)', 's_R (%)'],
    'labs': ['number of laboratories', 'laboratuvar sayısı'],
    'bias_percent': ['bias (%)', 'sapma (%)'],
    'recovery_percent': ['recovery (%)', 'geri kazanım (%)'],
    'mean_after': ['mean after spiking', 'katkı sonrası ortalama'],
    'sd_after': ['s after spiking', 'katkı sonrası s'],
    'n_after': ['n after spiking', 'katkı sonrası n'],
    'mean_before': ['mean before spiking', 'katkı öncesi ortalama'],
    'sd_before': ['s before spiking', 'katkı öncesi s'],
    'n_before': ['n before spiking', 'katkı öncesi n'],
    'spike': ['added amount c₊', 'eklenen miktar c₊'],
    'u_spike': ['u(c₊)', 'u(c₊)'],
    'u_spike_percent': ['u(c₊) (%)', 'u(c₊) (%)'],
    'mean_percent': ['mean recovery (%)', 'ortalama geri kazanım (%)'],
    'u': ['standard uncertainty u', 'standart belirsizlik u'],
    'u_percent': ['standard uncertainty (%)', 'standart belirsizlik (%)'],
    'half_width_percent': ['half-width a (%)', 'yarı genişlik a (%)'],
    'multiplicity': ['multiplicity', 'çokluk'],
    'sd_data': ['standard deviation s', 'standart sapma s'],
  };

  String parameter(String p) {
    final v = parameterLabels[p];
    if (v == null) return p;
    return tr ? v[1] : v[0];
  }
}
