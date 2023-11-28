use super::fields::m31::BaseField;

pub fn butterfly<F: Field>(v0: &mut F, v1: &mut F, twid: BaseField) {
    let tmp = *v1 * twid;
    *v1 = *v0 - tmp;
    *v0 += tmp;
}

pub fn ibutterfly<F: Field>(v0: &mut F, v1: &mut F, itwid: BaseField) {
    let tmp = *v0;
    *v0 = tmp + *v1;
    *v1 = (tmp - *v1) * itwid;
}
